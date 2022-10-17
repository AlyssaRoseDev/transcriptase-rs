#![allow(clippy::or_fun_call)]
use miette::{Diagnostic, NamedSource, SourceSpan};
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::{char, one_of},
    error::{VerboseError, VerboseErrorKind},
    number::complete::double,
    sequence::{terminated, tuple},
    IResult, Parser,
};
use nom_supreme::{
    final_parser::{final_parser, ExtractContext},
    ParserExt,
};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Gff Parsing Error")]
pub struct ParseError {
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
            "GFF Parsing Error"
        };
        ParseError {
            src: NamedSource::new(reason, original_input.to_string()),
            err_loc: original_input
                .find(fail)
                .expect("This error came from finding 'fail' in 'original_input'")
                .into(),
        }
    }
}

pub(crate) type NomResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub(crate) const RESERVED: &str = "\t\r\n";

pub(crate) fn undefined<T>(src: &str) -> NomResult<'_, Option<T>> {
    char('.').parse(src).map(|(rem, _)| (rem, None))
}

pub(crate) fn seq_id(src: &str) -> NomResult<'_, &str> {
    const VALID: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%>";
    is_a(VALID)
        .verify(|&id: &&str| !id.starts_with('>'))
        .context("Invalid Seq_Id Character")
        .parse(src)
}

#[test]
fn whatthedogdoin() -> miette::Result<()> {
    use nom::sequence::separated_pair;
    Ok((|ver| -> Result<(), ParseError> {
        final_parser(
            tag::<_, _, VerboseError<&str>>("3.")
                .context("Version must be at least 3.0")
                .and(separated_pair(
                    nom::character::complete::u8,
                    char('.'),
                    nom::character::complete::u8,
                ))
                .map(|_| ()),
        )(ver)
    })("2.0.00000000000000000000000")?)
}

pub(crate) fn source(src: &str) -> NomResult<'_, &str> {
    is_not(RESERVED).parse(src)
}

pub(crate) fn feature_type(src: &str) -> NomResult<'_, &str> {
    is_not(RESERVED).parse(src)
}

pub(crate) fn range_bound(src: &str) -> NomResult<'_, usize> {
    is_a("0123456789")
        .verify(|&bound: &&str| bound.parse::<usize>().is_ok())
        .parse(src)
        .map(|(rem, bound)| {
            (
                rem,
                bound
                    .parse::<usize>()
                    .expect("Verify only allows valid usizes"),
            )
        })
}

pub(crate) fn score(src: &str) -> NomResult<'_, Option<f64>> {
    undefined(src).or(double(src).map(|(rem, score)| (rem, Some(score))))
}

pub(crate) fn strand(src: &str) -> NomResult<'_, Option<char>> {
    const VALID: &str = "+-?";
    undefined(src).or(one_of(VALID)
        .parse(src)
        .map(|(rem, strand)| (rem, Some(strand))))
}

pub(crate) fn phase(src: &str) -> NomResult<'_, Option<u8>> {
    const VALID: &str = "012";
    undefined(src).or(one_of(VALID).parse(src).map(|(rem, phase)| {
        let phase = match phase {
            '0' => 0u8,
            '1' => 1u8,
            '2' => 2u8,
            _ => unreachable!(),
        };
        (rem, Some(phase))
    }))
}

pub(crate) fn attributes(src: &str) -> NomResult<'_, &str> {
    const VALID: &str =
        r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%=, "'()/;"#;
    is_a(VALID).parse(src)
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
