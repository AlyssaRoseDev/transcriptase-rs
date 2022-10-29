use miette::{Diagnostic, NamedSource, SourceSpan};
use nom::{
    bytes::complete::is_a,
    character::complete::{line_ending, multispace0, not_line_ending, one_of},
    error::{VerboseError, VerboseErrorKind},
    multi::many1,
    sequence::{delimited, pair},
    Parser,
};
use nom_supreme::{
    final_parser::{final_parser, ExtractContext},
    ParserExt,
};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::{iter::FromIterator, ops::Index};
use thiserror::Error;

use crate::NomResult;

#[derive(Debug, Error, Diagnostic)]
#[error("FASTA Parsing Error: {msg}")]
pub struct FastaError {
    msg: Box<str>,
    #[source_code]
    src: NamedSource,
    #[label("Here")]
    err_loc: SourceSpan,
}

impl ExtractContext<&str, FastaError> for VerboseError<&str> {
    fn extract_context(self, original_input: &str) -> FastaError {
        let kind = &self.errors[0].1;
        let (fail, ctx) = &self.errors[1];
        let reason = if let VerboseErrorKind::Context(ctx) = ctx {
            ctx
        } else if let VerboseErrorKind::Nom(e) = kind {
            e.description()
        } else {
            "Unknown Error Kind; This is a bug and should be reported!"
        };
        FastaError {
            msg: reason.into(),
            src: NamedSource::new(reason, original_input.to_string()),
            err_loc: original_input
                .find(fail)
                .expect("This error came from finding 'fail' in 'original_input'")
                .into(),
        }
    }
}

/// [`Fasta`] is a simple text-based format for genomic and proteomic sequences that stores an optional
/// description and a sequence of [`RNA`](crate::genomics::nucleotide::RNA), [`DNA`](crate::genomics::nucleotide::DNA), or [`Amino Acids`](crate::proteomics::amino::AminoAcid).
#[derive(Debug, Clone)]
pub struct Fasta<T>
where
    T: Sequence,
{
    /// The description given in the inital comment line
    pub description: Option<Box<str>>,
    /// The genomic or proteomic sequence
    pub sequence: T,
}

impl<T> Fasta<T>
where
    T: Sequence,
{
    /// Parses a string slice as a [`Fasta`] formatted document.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Sequence`]
    /// parse implementation returns an error
    #[cfg(feature = "rayon")]
    pub fn parse(src: &str) -> Result<Vec<Self>, FastaError> {
        use tracing::trace;

        final_parser(many1(pair(comment_line, sequence_block).map(
            |(description, sequence)| {
                trace!("Successfully parsed a FASTA block");
                Self {
                    description,
                    sequence,
                }
            },
        )))(src)
    }
}

fn comment_line(src: &str) -> NomResult<'_, Option<Box<str>>> {
    delimited(one_of(">;"), not_line_ending, line_ending)
        .opt()
        .map(|s| s.map(Into::into))
        .parse(src)
}

fn sequence_line<T>(src: &str) -> NomResult<'_, &str>
where
    T: Sequence,
{
    is_a(T::VALID_CHARS)
        .terminated(multispace0)
        .context("Sequence Line contained invalid character")
        .parse(src)
}

#[cfg(not(feature = "rayon"))]
fn sequence_block<T>(src: &str) -> NomResult<'_, T>
where
    T: Sequence,
{
    many1(sequence_line::<T>)
        .all_consuming()
        .map(|lines| {
            lines
                .iter()
                .flat_map(|s| {
                    s.chars().map(|c| {
                        T::Inner::try_from(c)
                            .expect("parser prevents us from reaching here with invalid characters")
                    })
                })
                .collect::<T>()
        })
        .context("Failed to parse sequence block")
        .parse(src)
}

#[cfg(feature = "rayon")]
fn sequence_block<T>(src: &str) -> NomResult<'_, T>
where
    T: Sequence,
{
    many1(sequence_line::<T>)
        .all_consuming()
        .map(|lines| {
            lines
                .par_iter()
                .flat_map(|s| {
                    s.par_chars().map(|c| {
                        T::Inner::try_from(c)
                            .expect("parser prevents us from reaching here with invalid characters")
                    })
                })
                .collect::<T>()
        })
        .context("Failed to parse sequence block")
        .parse(src)
}

#[cfg(not(feature = "rayon"))]
pub trait Sequence
where
    Self: Index<usize> + Extend<Self::Inner> + FromIterator<Self::Inner> + Sized,
{
    const VALID_CHARS: &'static str;

    /// The type of each member of the sequence
    type Inner: TryFrom<char, Error = String>;

    /// Serialize this `Sequence` to a text format
    fn serialize(&self) -> String;

    /// Serialize this `Sequence` to a raw binary stream
    fn serialize_bytes(&self) -> &[u8];
}

/// A sequence that can be:
/// - Parsed from a text format
/// - Serialized to a text format
/// - Serialized to a raw binary representation
#[cfg(feature = "rayon")]
pub trait Sequence
where
    Self: Index<usize> + Extend<Self::Inner> + FromIterator<Self::Inner> + Sized,
    Self: FromParallelIterator<Self::Inner>,
{
    const VALID_CHARS: &'static str;

    /// The type of each member of the sequence
    type Inner: TryFrom<char, Error = String> + Send;

    /// Serialize this `Sequence` to a text format
    fn serialize(&self) -> String;

    /// Serialize this `Sequence` to a raw binary stream
    fn serialize_bytes(&self) -> &[u8];
}
