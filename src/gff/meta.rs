use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Range,
};

use super::{GffError, UnescapedString};
use crate::gff::parsers::ParseError;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::char,
    combinator::map,
    error::VerboseError,
    sequence::{preceded, separated_pair},
};
use nom_supreme::{final_parser::final_parser, ParserExt};

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
    #[tracing::instrument]
    pub(crate) fn parse_metadata(&mut self, line: &str) -> Result<(), GffError> {
        let (kind, rem) = line.split_once(' ').ok_or(GffError::MalformedLine)?;
        match kind {
            "gff-version" => self.version = Some(Metadata::parse_version(rem)?),
            "sequence-region" => {
                let (seq_id, range) = Metadata::parse_sequence_region(rem)?;
                if let Some(ref mut map) = self.sequence_regions {
                    match map.entry(UnescapedString::new(seq_id)?) {
                        Entry::Occupied(_) => {
                            return Err(GffError::DuplicateSequence);
                        }
                        Entry::Vacant(e) => e.insert(range),
                    };
                } else {
                    self.sequence_regions =
                        Some(HashMap::from([(UnescapedString::new(seq_id)?, range)]));
                }
            }
            //TODO: Add proper uri parsing to ontology & species metaattributes
            "feature-ontology" => self.feature_ontology_uri = Some(UnescapedString::new(rem)?),
            "attribute-ontology" => self.attribute_ontology_uri = Some(UnescapedString::new(rem)?),
            "source-ontology" => self.source_ontology_uri = Some(UnescapedString::new(rem)?),
            "species" => self.species_uri = Some(UnescapedString::new(rem)?),
            "genome-build" => {
                let genome_build = rem
                    .split_once(' ')
                    .ok_or(GffError::InvalidGenomeBuild)
                    .and_then(|(l, r)| Ok((UnescapedString::new(l)?, UnescapedString::new(r)?)))?;
                self.genome_build = Some(genome_build);
            }
            _ => return Err(GffError::InvalidAttribute),
        };
        Ok(())
    }

    pub(crate) fn parse_domain_metadata(&mut self, line: &str) -> Result<(), GffError> {
        let (key, val) = line.split_once(' ').ok_or(GffError::MalformedLine)?;
        if let Some(ref mut map) = self.other_meta {
            match map.entry(UnescapedString::new(key)?) {
                Entry::Occupied(_) => {
                    return Err(GffError::DuplicateMetaAttribute);
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

    fn parse_version(ver: &str) -> Result<(u8, u8), GffError> {
        final_parser::<_, _, VerboseError<_>, ParseError>(preceded(
            tag("3."),
            separated_pair(
                nom::character::complete::u8,
                char('.'),
                nom::character::complete::u8,
            ),
        ))(ver)
        .map_err(Into::into)
    }

    fn parse_sequence_region(seq_region: &str) -> Result<(&str, Range<u64>), GffError> {
        const VALID: &str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%>";
        final_parser::<_, _, VerboseError<&str>, ParseError>(map(
            separated_pair(
                is_a(VALID).verify(|&id: &&str| !id.starts_with('>')),
                char(' '),
                separated_pair(
                    nom::character::complete::u64,
                    char(' '),
                    nom::character::complete::u64,
                ),
            ),
            |(seq_id, (start, end))| (seq_id, start..end),
        ))(seq_region)
        .map_err(Into::into)
    }
}
