use std::str::FromStr;

use crate::err::{TXaseError, TXaseResult};
use attr::{Attribute, Id};
use either::Either;
use nom::{bytes::complete::is_a, Parser};

mod attr;
mod parsers;
#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct GFF {
    meta: Vec<Metadata>,
    entries: Vec<Entry>,
}

impl GFF {
    pub fn parse(src: &impl AsRef<str>) -> TXaseResult<Self> {
        fn inner(this: &str) -> TXaseResult<GFF> {
            let mut meta = Vec::new();
            let mut entries = Vec::new();
            for line in this.lines() {
                if line.starts_with('#') {
                    if let Some(metadata) = Metadata::parse(line)? {
                        meta.push(metadata);
                    }
                } else {
                    entries.push(Entry::parse(line)?);
                }
            }

            Ok(GFF { meta, entries })
        }
        inner(src.as_ref())
    }

    #[must_use]
    pub fn metadata(&self) -> &Vec<Metadata> {
        &self.meta
    }

    #[must_use]
    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

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
    pub range: (usize, usize),
    pub score: Option<f64>,
    pub strand: Option<Strand>,
    pub phase: Option<u8>,
    pub id: Option<Id>,
    pub attrs: Vec<Attribute>,
}

impl Entry {
    // GFF Entry line:
    // {seq_id} {source} {type} {start} {end} {score?} {strand} {phase?} {attributes[]}
    pub(crate) fn parse(src: &str) -> TXaseResult<Self> {
        let (_, raw) = parsers::entry(src)?;
        let (seq, source, feature_type, range_start, range_end, score, strand, phase, attributes) =
            raw;
        let mut id = None;
        let mut attrs = Vec::new();
        for &attr in &attributes {
            match Attribute::parse(attr)? {
                Either::Left(attribute) => attrs.push(attribute),
                Either::Right(id_attr) => {
                    if id.is_none() {
                        id = Some(id_attr);
                    } else {
                        return Err(TXaseError::DuplicateGFFEntryID());
                    }
                }
            }
        }
        Ok(Self {
            seq_id: UnescapedString::new(seq)?,
            source: UnescapedString::new(source)?,
            feature_type: UnescapedString::new(feature_type)?,
            range: (range_start, range_end),
            score,
            strand: strand.map(Strand::parse).transpose()?,
            phase,
            id,
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
