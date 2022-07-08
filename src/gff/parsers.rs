use nom::{
    bytes::complete::{is_a, tag, take_until1},
    character::complete::{char, one_of},
    number::complete::double,
    sequence::{terminated, tuple},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

pub(crate) fn undefined<T>(src: &str) -> IResult<&str, Option<T>, ErrorTree<&str>> {
    char::<&str, ErrorTree<&str>>('.')
        .parse(src)
        .map(|(rem, _)| (rem, None))
}

pub(crate) fn seq_id(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    const VALID: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%";
    is_a(VALID)
        .verify(|&id: &&str| !id.starts_with('>'))
        .parse(src)
}

pub(crate) fn source(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    take_until1("\t").parse(src)
}

pub(crate) fn feature_type(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    const VALID: &str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%";
    is_a(VALID).parse(src)
}

pub(crate) fn range_bound(src: &str) -> IResult<&str, usize, ErrorTree<&str>> {
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

pub(crate) fn score(src: &str) -> IResult<&str, Option<f64>, ErrorTree<&str>> {
    undefined(src).or_else(|_| double(src).map(|(rem, score)| (rem, Some(score))))
}

pub(crate) fn strand(src: &str) -> IResult<&str, Option<char>, ErrorTree<&str>> {
    const VALID: &str = "+-.?";
    undefined(src).or_else(|_| {
        one_of(VALID)
            .parse(src)
            .map(|(rem, strand)| (rem, Some(strand)))
    })
}

pub(crate) fn phase(src: &str) -> IResult<&str, Option<u8>, ErrorTree<&str>> {
    const VALID: &str = "012";
    undefined(src).or_else(|_| {
        one_of(VALID).parse(src).map(|(rem, phase)| {
            let phase = match phase {
                '0' => 0u8,
                '1' => 1u8,
                '2' => 2u8,
                _ => unreachable!(),
            };
            (rem, Some(phase))
        })
    })
}

pub(crate) fn attributes(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
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

pub(crate) fn entry(src: &str) -> IResult<&str, RawEntry<'_>, ErrorTree<&str>> {
    tuple((
        terminated(seq_id, tag("\t")),
        terminated(source, tag("\t")),
        terminated(feature_type, tag("\t")),
        terminated(range_bound, tag("\t")),
        terminated(range_bound, tag("\t")),
        terminated(score, tag("\t")),
        terminated(strand, tag("\t")),
        terminated(phase, tag("\t")),
        attributes,
    ))
    .all_consuming()
    .parse(src)
}
