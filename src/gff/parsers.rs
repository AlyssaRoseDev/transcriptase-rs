use nom::{
    bytes::complete::{is_a, tag, take_until1},
    character::complete::{char, digit1, one_of},
    multi::separated_list0,
    number::complete::double,
    IResult, Parser,
};
use nom_supreme::{error::*, ParserExt};
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

pub(crate) fn range_bound(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    digit1(src)
}

pub(crate) fn score(src: &str) -> IResult<&str, Option<f64>, ErrorTree<&str>> {
    if let Ok((rem, _)) = char::<&str, ErrorTree<&str>>('.').parse(src) {
        return Ok((rem, None));
    }
    double(src).map(|(rem, score)| (rem, Some(score)))
}

pub(crate) fn strand(src: &str) -> IResult<&str, char, ErrorTree<&str>> {
    const VALID: &str = "+-.?";
    one_of(VALID).parse(src)
}

pub(crate) fn phase(src: &str) -> IResult<&str, Option<char>, ErrorTree<&str>> {
    const VALID: &str = "012";
    if let Ok((rem, _)) = char::<&str, ErrorTree<&str>>('.').parse(src) {
        return Ok((rem, None));
    }
    one_of(VALID)
        .parse(src)
        .map(|(rem, phase)| (rem, Some(phase)))
}

pub(crate) fn attributes(src: &str) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    const VALID: &str =
        r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.:^*$@!+_?-|%=, "'()/"#;
    separated_list0(tag(";"), is_a(VALID)).parse(src)
}
