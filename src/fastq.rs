use std::collections::HashMap;

use crate::fasta::Sequence;
use crate::NomResult;
use miette::{Diagnostic, NamedSource, SourceSpan};
use nom::character::complete::multispace0;
use nom::error::{VerboseError, VerboseErrorKind};
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::Parser;
use nom_supreme::final_parser::{final_parser, ExtractContext};
use nom_supreme::ParserExt;
use quality::Quality;
pub use quality::{Phred, Solexa};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use thiserror::Error;
use tracing::trace;

pub mod quality;

pub type Descriptor = String;
pub type QualitySequence<T, Q> = Vec<(T, Q)>;

#[derive(Debug, Error, Diagnostic)]
pub enum FastQError {
    #[error("Descriptions of FastQ file did not match!")]
    MismatchedDescription,
    #[error("File contained no FastQ data")]
    EmptyFile,
    #[error("{msg}")]
    ParsingError {
        msg: Box<str>,
        #[source_code]
        src: NamedSource,
        #[label("Here")]
        err_loc: SourceSpan,
    },
}

impl ExtractContext<&str, FastQError> for VerboseError<&str> {
    fn extract_context(self, original_input: &str) -> FastQError {
        let (fail, kind) = &self.errors[0];
        let ctx = &self.errors.get(1);
        let reason = if let Some((_, ctx_err)) = ctx {
            if let VerboseErrorKind::Context(ctx_msg) = ctx_err {
                ctx_msg
            } else {
                unreachable!()
            }
        } else if let VerboseErrorKind::Nom(e) = kind {
            e.description()
        } else {
            "Unknown Error Kind; This is a bug and should be reported!"
        };
        trace!(fail);
        let err_loc = original_input
            .find(fail)
            .expect("This error came from finding 'fail' in 'original_input'");
        FastQError::ParsingError {
            msg: reason.into(),
            src: NamedSource::new(reason, original_input.to_string()),
            err_loc: err_loc.into(),
        }
    }
}

#[derive(Debug)]
pub struct FastQ<S, Q>
where
    S: Sequence,
    Q: Quality,
{
    pub sequences: HashMap<Descriptor, QualitySequence<S::Inner, Q>>,
}

#[cfg(not(feature = "rayon"))]
impl<S, Q> FastQ<S, Q>
where
    S: Sequence,
    Q: Quality,
{
    #[tracing::instrument(skip_all)]
    pub fn parse(src: &str) -> Result<Self, FastQError> {
        let sequences = final_parser(
            delimited(multispace0, many1(Self::parse_single), multispace0)
                .context("FastQ files must contain at least one entry"),
        )(src)?
        .into_iter()
        .collect::<_>();
        Ok(Self { sequences })
    }

    fn parse_single(src: &str) -> NomResult<'_, (Descriptor, QualitySequence<S::Inner, Q>)> {
        tuple((
            parsers::desc_line,
            parsers::sequence_line::<S>,
            parsers::optional_desc_line,
            parsers::quality_line::<Q>,
        ))
        .context("Incomplete FastQ block")
        .verify(|(desc, _, desc2, _)| {
            if let Some(desc2) = desc2 {
                desc2.is_empty() || desc == desc2
            } else {
                true
            }
        })
        .context(
            "FastQ entries must have matching descriptions if the second description is non-empty",
        )
        .map(|(desc, seq_line, _, qual_line)| {
            (
                desc.to_string(),
                seq_line
                    .chars()
                    .zip(qual_line.chars())
                    .map(|(s, q)| {
                        (
                            S::Inner::try_from(s).expect(
                                "Parser prevents us from reaching here with invalid characters",
                            ),
                            Q::try_from(q).expect(
                                "Parser prevents us from reaching here with invalid characters",
                            ),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .parse(src)
    }
}

#[cfg(feature = "rayon")]
impl<S, Q> FastQ<S, Q>
where
    S: Sequence,
    Q: Quality + Send,
    S::Inner: Send,
{
    #[tracing::instrument(skip_all)]
    pub fn parse(src: &str) -> Result<Self, FastQError> {
        let sequences = final_parser(
            delimited(multispace0, many1(Self::parse_single), multispace0)
                .context("FastQ files must contain at least one entry"),
        )(src)?
        .into_par_iter()
        .collect::<_>();
        Ok(Self { sequences })
    }

    //#[tracing::instrument(skip_all)]
    fn parse_single(src: &str) -> NomResult<'_, (Descriptor, QualitySequence<S::Inner, Q>)> {
        tuple((
            parsers::desc_line,
            parsers::sequence_line::<S>,
            parsers::optional_desc_line,
            parsers::quality_line::<Q>,
        ))
        .context("Incomplete FastQ block")
        .verify(|(desc, _, desc2, _)| {
            if let Some(desc2) = desc2 {
                desc2.is_empty() || desc == desc2
            } else {
                true
            }
        })
        .context(
            "FastQ entries must have matching descriptions if the second description is non-empty",
        )
        .map(|(desc, seq_line, _, qual_line)| {
            (
                desc.to_string(),
                seq_line
                    .as_bytes()
                    .into_par_iter()
                    .zip(qual_line.as_bytes().into_par_iter())
                    .map(|(&s, &q)| {
                        (
                            S::Inner::try_from(char::from(s)).expect(
                                "Parser prevents us from reaching here with invalid characters",
                            ),
                            Q::try_from(char::from(q)).expect(
                                "Parser prevents us from reaching here with invalid characters",
                            ),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .parse(src)
    }
}

mod parsers {
    use nom::{
        bytes::complete::{is_a, take_while},
        character::complete::{line_ending, not_line_ending},
        combinator::opt,
        sequence::delimited,
        Parser,
    };
    use nom_supreme::{tag::complete::tag, ParserExt};

    use crate::{fasta::Sequence, NomResult};

    use super::quality::Quality;

    pub fn desc_line(src: &str) -> NomResult<'_, &str> {
        delimited(tag("@"), not_line_ending, line_ending)
            .context("First description line was malformed")
            .parse(src)
    }

    pub fn sequence_line<T: Sequence>(src: &str) -> NomResult<'_, &str> {
        is_a(T::VALID_CHARS)
            .terminated(line_ending)
            .context("Sequence line contained invalid characters")
            .parse(src)
    }

    pub fn optional_desc_line(src: &str) -> NomResult<'_, Option<&str>> {
        delimited(tag("+"), opt(not_line_ending), line_ending)
            .context("Second description line was malformed")
            .parse(src)
    }

    pub fn quality_line<Q: Quality>(src: &str) -> NomResult<'_, &str> {
        take_while(Q::is_valid)
            .terminated(line_ending)
            .context("Quality line contained invalid characters")
            .parse(src)
    }
}
