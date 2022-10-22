use std::{fmt, io::Read, ops::Range, str::FromStr};

use attr::AttributeSet;
use meta::Metadata;
use miette::Diagnostic;
use nom::{bytes::complete::is_a, Parser};
use thiserror::Error;

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
    #[error("{0}")]
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
#[derive(Debug, Clone)]
pub struct GFF {
    /// A list of the [`Metadata`]
    pub metadata: Metadata,
    /// A list of the entries
    pub entries: Vec<Entry>,
}

impl GFF {
    /// Attempts to parse the given [`Reader`](std::io::Read) as a GFFv3-formatted input
    #[tracing::instrument(skip_all)]
    pub fn parse(src: &mut impl Read) -> Result<Self, GffError> {
        let mut temp = String::new();
        src.read_to_string(&mut temp)?;
        temp.parse()
    }
}

impl FromStr for GFF {
    type Err = GffError;

    #[tracing::instrument(skip_all)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut metadata = Metadata::default();
        let mut entries = Vec::new();
        for line in s.lines() {
            if let Ok((tag, meta)) = is_a::<_, _, ()>("#").parse(line) {
                match tag {
                    "###" => break,
                    "##" => metadata.parse_metadata(meta)?,
                    "#" if meta.starts_with('!') => metadata.parse_domain_metadata(meta)?,
                    _ => {
                        continue;
                    }
                }
            } else {
                entries.push(Entry::from_str(line)?)
            }
        }

        Ok(GFF { metadata, entries })
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
    pub(crate) fn parse(src: &str) -> Result<Self, GffError> {
        // GFF Entry line:
        // {seq_id} {source} {type} {start} {end} {score?} {strand} {phase?} {attributes[]}
        let (seq, source, feature_type, range_start, range_end, score, strand, phase, attributes) =
            parsers::entry(src)?;
        let attrs = AttributeSet::parse(attributes)?;
        Ok(Self {
            seq_id: UnescapedString::new(seq)?,
            source: UnescapedString::new(source)?,
            feature_type: UnescapedString::new(feature_type)?,
            range: range_start..range_end,
            score,
            strand: strand.map(Strand::parse).transpose()?,
            phase,
            attrs,
        })
    }
}

impl FromStr for Entry {
    type Err = GffError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Entry::parse(s)
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
    #[tracing::instrument(name = "UnescapedString::new")]
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
        f.write_str(self.0.as_ref())
    }
}
