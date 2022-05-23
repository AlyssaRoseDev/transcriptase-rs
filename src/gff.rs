use std::str::FromStr;

use crate::err::{TXError, TXResult};
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
    pub fn parse(src: &impl AsRef<str>) -> TXResult<Self> {
        fn inner(this: &str) -> TXResult<GFF> {
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

    pub fn metadata(&self) -> &Vec<Metadata> {
        &self.meta
    }

    pub fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

#[derive(Debug, Clone)]
pub enum Metadata {
    Pragma(Box<str>),
    Other(Box<str>),
}

impl Metadata {
    pub(crate) fn parse(src: &str) -> TXResult<Option<Self>> {
        let (tag, meta) = is_a("#").parse(src)?;
        Ok(match tag {
            "##" => Some(Self::Pragma(meta.into())),
            "#" => Some(Self::Other(meta.into())),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub seq_id: Box<str>,
    pub source: Box<str>,
    pub feature_type: Box<str>,
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
    pub(crate) fn parse(src: &str) -> TXResult<Self> {
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
                        return Err(TXError::DuplicateGFFEntryID());
                    }
                }
            }
        }
        Ok(Self {
            seq_id: seq.into(),
            source: source.into(),
            feature_type: feature_type.into(),
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
    type Err = TXError;

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
    pub fn parse(src: char) -> TXResult<Self> {
        Ok(match src {
            '+' => Self::Positive,
            '-' => Self::Negative,
            '?' => Self::Unknown,
            _ => {
                return Err(TXError::NomParsing(format!(
                    "Unexpected Strand kind, expected one of: ['+', '-', '?'], got {src}"
                )))
            }
        })
    }
}
