use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Range,
};

use super::UnescapedString;
use crate::err::{TXaseError, TXaseResult};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::char,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub version: Option<(u8, u8)>,
    pub sequence_regions: Option<HashMap<UnescapedString, Range<u64>>>,
    pub feature_ontology_uri: Option<UnescapedString>,
    pub attribute_ontology_uri: Option<UnescapedString>,
    pub source_ontology_uri: Option<UnescapedString>,
    pub species_uri: Option<UnescapedString>,
    pub genome_build: Option<(UnescapedString, UnescapedString)>,
    pub other_meta: Option<HashMap<UnescapedString, UnescapedString>>,
}

impl Metadata {
    pub(crate) fn parse_metadata(&mut self, line: &str) -> TXaseResult<()> {
        let (kind, rem) = line.split_once(' ').ok_or_else(|| {
            TXaseError::InternalParseFailure(format!("GFF pragma did not contain data: {line}"))
        })?;
        match kind {
            "gff-version" => self.version = Some(Metadata::parse_version(rem)?),
            "sequence-region" => {
                let (seq_id, range) = Metadata::parse_sequence_region(rem)?;
                if let Some(ref mut map) = self.sequence_regions {
                    match map.entry(UnescapedString::new(seq_id)?) {
                        Entry::Occupied(_) => {
                            return Err(TXaseError::InternalParseFailure(format!(
                                "Duplicate key {seq_id} found"
                            )));
                        }
                        Entry::Vacant(e) => e.insert(range),
                    };
                } else {
                    self.sequence_regions =
                        Some(HashMap::from([(UnescapedString::new(seq_id)?, range)]));
                }
            }
            "feature-ontology" => todo!(),
            "attribute-ontology" => todo!(),
            "source-ontology" => todo!(),
            "species" => todo!(),
            "genome-build" => todo!(),
            _ => todo!(),
        };
        Ok(())
    }

    pub(crate) fn parse_domain_metadata(&mut self, line: &str) -> TXaseResult<()> {
        let (key, val) = line.split_once(' ').ok_or_else(|| {
            TXaseError::InternalParseFailure(format!("GFF pragma did not contain data: {line}"))
        })?;
        if let Some(ref mut map) = self.other_meta {
            match map.entry(UnescapedString::new(key)?) {
                Entry::Occupied(_) => {
                    return Err(TXaseError::InternalParseFailure(format!(
                        "Duplicate key {key} found"
                    )));
                }
                Entry::Vacant(e) => e.insert(UnescapedString::new(val)?),
            };
        } else {
            self.other_meta = Some(HashMap::from([(
                UnescapedString::new(key)?,
                UnescapedString::new(val)?,
            )]));
        }
        Ok(())
    }

    fn parse_version(ver: &str) -> TXaseResult<(u8, u8)> {
        tag("3.")
            .and(separated_pair(
                nom::character::complete::u8,
                char('.'),
                nom::character::complete::u8,
            ))
            .all_consuming()
            .parse(ver)
            .map(|(_, (_, ver))| ver)
            .map_err(Into::into)
    }

    fn parse_sequence_region(seq_region: &str) -> TXaseResult<(&str, Range<u64>)> {
        const VALID: &str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%>";
        separated_pair(
            is_a(VALID).verify(|&id: &&str| !id.starts_with('>')),
            char(' '),
            separated_pair(
                nom::character::complete::u64,
                char(' '),
                nom::character::complete::u64,
            ),
        )
        .all_consuming()
        .parse(seq_region)
        .map_err(Into::into)
        .and_then(|(_, (seq_id, (start, end)))| Ok((seq_id, start..end)))
    }
}
