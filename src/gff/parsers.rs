#![allow(clippy::or_fun_call)]
use miette::{Diagnostic, NamedSource, SourceSpan};
use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag},
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res},
    error::{VerboseError, VerboseErrorKind},
    number::complete::double,
    sequence::{terminated, tuple},
    Parser,
};
use nom_supreme::{
    final_parser::{final_parser, ExtractContext},
    ParserExt,
};
use thiserror::Error;

use crate::NomResult;

#[derive(Debug, Error, Diagnostic)]
#[error("Gff Parsing Error: {msg}")]
pub struct ParseError {
    msg: Box<str>,
    #[source_code]
    src: NamedSource,
    #[label("Here")]
    err_loc: SourceSpan,
}

impl ExtractContext<&str, ParseError> for VerboseError<&str> {
    fn extract_context(self, original_input: &str) -> ParseError {
        let kind = &self.errors[0].1;
        let (fail, ctx) = &self.errors[1];
        let reason = if let VerboseErrorKind::Context(ctx) = ctx {
            ctx
        } else if let VerboseErrorKind::Nom(e) = kind {
            e.description()
        } else {
            "Unknown Error Kind; This is a bug and should be reported!"
        };
        ParseError {
            msg: reason.into(),
            src: NamedSource::new(reason, original_input.to_string()),
            err_loc: original_input
                .find(fail)
                .expect("This error came from finding 'fail' in 'original_input'")
                .into(),
        }
    }
}

pub(crate) const RESERVED: &str = "\t\r\n";

pub(crate) fn seq_id(src: &str) -> NomResult<'_, &str> {
    const VALID: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%>";
    is_a(VALID)
        .verify(|&id: &&str| !id.starts_with('>'))
        .context("Invalid Seq_Id Character")
        .parse(src)
}

pub(crate) fn source(src: &str) -> NomResult<'_, &str> {
    is_not(RESERVED)
        .context("Reserved Character in Source")
        .parse(src)
}

pub(crate) fn feature_type(src: &str) -> NomResult<'_, &str> {
    is_not(RESERVED)
        .context("Reserved Character in Feature Type")
        .parse(src)
}

pub(crate) fn range_bound(src: &str) -> NomResult<'_, usize> {
    map_res(digit1, str::parse)
        .context("Non-number input found in range bound")
        .parse(src)
}

pub(crate) fn score(src: &str) -> NomResult<'_, Option<f64>> {
    alt((map(char('.'), |_| None), map(double, Some)))
        .context("Invalid score, expected one of '.' or a valid floating point number")
        .parse(src)
}

pub(crate) fn strand(src: &str) -> NomResult<'_, Option<char>> {
    const VALID: &str = "+-?";
    alt((map(char('.'), |_| None), map(one_of(VALID), Some)))
        .context("Invalid strand, expected one of ['.', '+', '-', '?']")
        .parse(src)
}

pub(crate) fn phase(src: &str) -> NomResult<'_, Option<u8>> {
    const VALID: &str = "012";
    alt((
        map(char('.'), |_| None),
        map(one_of(VALID), |d| {
            Some(
                d.to_digit(10)
                    .expect("one_of(\"012\") always returns a base 10 digit")
                    .try_into()
                    .expect("one_of(\"012\") will always fit in a u8"),
            )
        }),
    ))
    .context("Invalid score, expected one of ['.', '0', '1', '2']")
    .parse(src)
}

pub(crate) fn attributes(src: &str) -> NomResult<'_, &str> {
    const VALID: &str =
        r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%=, "'()/;"#;
    is_a(VALID)
        .context("Attributes contained an invalid character")
        .parse(src)
}

type RawEntry<'a> = (
    &'a str,
    &'a str,
    &'a str,
    usize,
    usize,
    Option<f64>,
    Option<char>,
    Option<u8>,
    &'a str,
);

pub(crate) fn entry(src: &str) -> Result<RawEntry<'_>, ParseError> {
    final_parser(tuple((
        terminated(seq_id, tag("\t")),
        terminated(source, tag("\t")),
        terminated(feature_type, tag("\t")),
        terminated(range_bound, tag("\t")),
        terminated(range_bound, tag("\t")),
        terminated(score, tag("\t")),
        terminated(strand, tag("\t")),
        terminated(phase, tag("\t")),
        attributes,
    )))(src)
}
