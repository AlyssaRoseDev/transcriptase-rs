use nom::{bytes::complete::is_a, character::complete::one_of, IResult, Parser};
use nom_supreme::error::ErrorTree;

pub(crate) fn comment(src: &str) -> IResult<&str, (), ErrorTree<&str>> {
    one_of(">;").parse(src).map(|(rem, _)| (rem, ()))
}

pub(crate) fn codon(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    const VALID: &str = "0ACMGRSVTUWYHKDBN";
    is_a(VALID).parse(src)
}
