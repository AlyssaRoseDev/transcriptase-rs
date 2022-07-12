use std::{io::Read, str::FromStr, collections::HashMap, ops::Range};

use crate::err::{TXaseError, TXaseResult};
use attr::AttributeSet;
use either::Either;
use nom::{bytes::complete::is_a, Parser};

pub mod attr;
mod parsers;
#[cfg(test)]
mod test;

/// A Generic Feature Format Version 3 file including both metadata and entries
#[derive(Debug, Clone)]
pub struct GFF {
    /// A list of the [`Metadata`]
    pub meta: Vec<Metadata>,
    /// A list of the entries
    pub entries: Vec<Entry>,
}

impl GFF {
    /// Attempts to parse the given [`Reader`](std::io::Read) as a GFFv3-formatted input
    pub fn parse(src: &mut impl Read) -> TXaseResult<Self> {
        let mut temp = String::new();
        src.read_to_string(&mut temp)?;
        temp.parse()
    }
}

impl FromStr for GFF {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut meta = Vec::new();
        let mut entries = Vec::new();
        for line in s.lines() {
            match Metadata::parse(line) {
                Ok(Some(metadata)) => meta.push(metadata),
                Ok(None) => continue,
                Err(_) => entries.push(Entry::parse(line)?),
            }
        }

        Ok(GFF { meta, entries })
    }
}

pub struct Meta;

#[derive(Debug, Clone)]
pub enum Metadata {
    Pragma(UnescapedString),
    Other(UnescapedString),
}

impl Metadata {
    pub(crate) fn parse(src: &str) -> TXaseResult<Option<Self>> {
        let (tag, meta) = is_a("#").parse(src)?;
        Ok(match tag {
            "##" => Some(Self::Pragma(UnescapedString::new(meta)?)),
            "#" => Some(Self::Other(UnescapedString::new(meta)?)),
            _ => None,
        })
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
    pub(crate) fn parse(src: &str) -> TXaseResult<Self> {
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
    type Err = TXaseError;

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
    pub fn parse(src: char) -> TXaseResult<Self> {
        Ok(match src {
            '+' => Self::Positive,
            '-' => Self::Negative,
            '?' => Self::Unknown,
            _ => {
                return Err(TXaseError::InternalParseFailure(format!(
                    "Unexpected Strand kind, expected one of: ['+', '-', '?'], got {src}"
                )))
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnescapedString(Box<str>);

impl UnescapedString {
    pub fn new(src: &str) -> TXaseResult<Self> {
        if src.contains('%') {
            let mut escaped = src.to_owned();
            while let Some(at) = escaped.find('%') {
                let old = &escaped[at..][..3];
                let byte = &[u8::from_str_radix(&old[1..3], 16)?];
                let new = std::str::from_utf8(byte)?;
                escaped = escaped.replace(old, new);
            }
            Ok(Self(escaped.into_boxed_str()))
        } else {
            Ok(Self(src.into()))
        }
    }
}
