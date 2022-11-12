use std::{
    fmt,
    io::Read,
    ops::{Deref, DerefMut, Range},
    str::FromStr,
};

use attr::AttributeSet;
use meta::Metadata;
use miette::Diagnostic;
use nom::{
    branch::alt,
    character::complete::{line_ending, not_line_ending},
    combinator::{map, map_res, value},
    error::VerboseError,
    sequence::{delimited, tuple},
    Parser,
};
use nom_supreme::{
    final_parser::final_parser, multi::parse_separated_terminated_res, tag::complete::tag,
};
use thiserror::Error;

use crate::NomResult;

use self::parsers::ParseError;

pub mod attr;
pub mod meta;
mod parsers;
#[cfg(test)]
mod test;

#[derive(Debug, Error, Diagnostic)]
pub enum GffError {
    #[error("Invalid Attribute found")]
    InvalidAttribute,
    #[error("Sequence ID must be unique")]
    DuplicateSequence,
    #[error("Invalid Genome Build")]
    InvalidGenomeBuild,
    #[error(transparent)]
    ParseError(#[from] parsers::ParseError),
    #[error("Failed to decode an escaped string")]
    StringDecodeErr,
    #[error("Strand must be one of [. - + ?]")]
    InvalidStrand,
    #[error(transparent)]
    IoErr(#[from] std::io::Error),
    #[error("Most Meta-Attributes must be unique")]
    DuplicateMetaAttribute,
    #[error("Line was malformed and parsing could not continue")]
    MalformedLine,
    #[error("Failed to parse a value from string")]
    FromStrErr,
    #[error("Gap kind must be one of [M, I, D, F, R]")]
    InvalidGapKind,
    #[error("IsCircular must be a boolean")]
    IsCircularNotBool,
    #[error("Attributes starting with a capital letter are reserved.")]
    ReservedAttribute,
    #[error("Target Attribute must be in the form [target_id start end strand]")]
    MalformedTarget,
}

/// A Generic Feature Format Version 3 file including both metadata and entries
#[derive(Debug, Default, Clone)]
pub struct GFF {
    /// A list of the [`Metadata`]
    pub metadata: Metadata,
    /// A list of the entries
    pub entries: Vec<Entry>,
}

impl GFF {
    /// Attempts to parse the given [`Reader`](std::io::Read) as a GFFv3-formatted input
    #[tracing::instrument(skip_all)]
    pub fn read_from(src: &mut impl Read) -> Result<Self, GffError> {
        std::io::read_to_string(src)?.parse()
    }

    fn parse(src: &str) -> Result<Self, GffError> {
        final_parser::<_, _, VerboseError<&str>, ParseError>(parse_separated_terminated_res(
            not_line_ending,
            line_ending,
            delimited(line_ending, tag("###"), line_ending),
            GFF::default,
            Self::parse_line,
        ))(src)
        .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    fn parse_line(mut self, line: &str) -> Result<Self, GffError> {
        final_parser::<_, _, VerboseError<&str>, ParseError>(alt((
            //parse meta
            map_res(
                tuple((
                    alt((value(false, tag("##")), value(true, tag("#!")))),
                    not_line_ending,
                )),
                |(is_domain, meta)| -> Result<(), GffError> {
                    self.metadata.parse_line(is_domain, meta)
                },
            ),
            map(Entry::parse, |entry| {
                self.entries.push(entry);
            }),
        )))(line)?;
        Ok(self)
    }
}

impl FromStr for GFF {
    type Err = GffError;

    #[tracing::instrument(skip_all)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
#[derive(Debug, Clone)]
pub struct Entry {
    pub seq_id: UnescapedString,
    pub source: UnescapedString,
    pub feature_type: UnescapedString,
    pub range: Range<usize>,
    pub score: Option<f64>,
    pub strand: Option<Strand>,
    pub phase: Option<u8>,
    pub attrs: AttributeSet,
}

impl Entry {
    #[tracing::instrument]
    pub(crate) fn parse(src: &str) -> NomResult<'_, Self> {
        // GFF Entry line:
        // {seq_id} {source} {type} {start} {end} {score?} {strand?} {phase?} {attributes[]}
        map_res(
            parsers::entry,
            |(seq, source, feature, range_s, range_e, score, strand, phase, attrs)| -> Result<_, GffError> {
                Ok(Self {
                    seq_id: UnescapedString::new(seq)?,
                    source: UnescapedString::new(source)?,
                    feature_type: UnescapedString::new(feature)?,
                    range: range_s..range_e,
                    score,
                    strand: strand.map(Strand::parse).transpose()?,
                    phase,
                    attrs: AttributeSet::parse(attrs)?,
                })
            },
        ).parse(src)
    }
}

impl FromStr for Entry {
    type Err = GffError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(final_parser::<_, _, VerboseError<&str>, ParseError>(
            Entry::parse,
        )(s)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Strand {
    Positive,
    Negative,
    Unknown,
}

impl Strand {
    #[tracing::instrument(name = "Strand::parse")]
    pub fn parse(src: char) -> Result<Self, GffError> {
        Ok(match src {
            '+' => Self::Positive,
            '-' => Self::Negative,
            '?' => Self::Unknown,
            _ => return Err(GffError::InvalidStrand),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnescapedString(Box<str>);

impl UnescapedString {
    #[tracing::instrument(name = "UnescapedString::new", level = "trace")]
    pub fn new(src: &str) -> Result<Self, GffError> {
        if src.contains('%') {
            let mut escaped = src.to_owned();
            while let Some(at) = escaped.find('%') {
                let old = &escaped[at..][..3];
                let byte = &[
                    u8::from_str_radix(&old[1..3], 16).map_err(|_| GffError::StringDecodeErr)?
                ];
                let new = std::str::from_utf8(byte).map_err(|_| GffError::StringDecodeErr)?;
                escaped = escaped.replace(old, new);
            }
            Ok(Self(escaped.into_boxed_str()))
        } else {
            Ok(Self(src.into()))
        }
    }
}

impl fmt::Display for UnescapedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl AsRef<str> for UnescapedString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsMut<str> for UnescapedString {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut()
    }
}

impl Deref for UnescapedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for UnescapedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
