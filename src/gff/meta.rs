use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Range,
};

use super::{GffError, UnescapedString};
use crate::gff::parsers::ParseError;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{char, u8 as parse_u8},
    combinator::{map, value},
    error::VerboseError,
    sequence::{preceded, separated_pair},
};
use nom_supreme::{final_parser::final_parser, tag::complete::tag, ParserExt};

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
    #[tracing::instrument(skip(self))]
    pub(crate) fn parse_line(&mut self, domain: bool, src: &str) -> Result<(), GffError> {
        if domain {
            self.parse_domain_metadata(src)
        } else {
            self.parse_metadata(src)
        }
    }

    #[tracing::instrument(skip(self))]
    pub(crate) fn parse_metadata(&mut self, line: &str) -> Result<(), GffError> {
        let (kind, rem) = line.split_once(' ').ok_or(GffError::MalformedLine)?;
        match kind {
            "gff-version" => self.version = Metadata::parse_version(rem)?,
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

    #[tracing::instrument(skip(self))]
    pub(crate) fn parse_domain_metadata(&mut self, line: &str) -> Result<(), GffError> {
        let (key, val) = line.split_once(' ').ok_or(GffError::MalformedLine)?;
        if key == "spec-version" {
            self.version = Some(final_parser::<_, _, VerboseError<&str>, ParseError>(
                separated_pair(parse_u8, char('.'), parse_u8),
            )(val)?);
            return Ok(());
        }
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

    fn parse_version(ver: &str) -> Result<Option<(u8, u8)>, ParseError> {
        final_parser::<_, _, VerboseError<&str>, ParseError>(alt((
            preceded(
                tag("3."),
                map(separated_pair(parse_u8, char('.'), parse_u8), Some),
            ),
            value(None, tag("3")),
        )))(ver)
    }

    fn parse_sequence_region(seq_region: &str) -> Result<(&str, Range<u64>), ParseError> {
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
    }
}
