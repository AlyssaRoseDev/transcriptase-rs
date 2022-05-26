use crate::err::{TXError, TXResult};
use nom::{
    bytes::complete::take_until,
    character::complete::{newline, one_of},
    sequence::delimited,
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use std::ops::Index;

pub struct Fasta<T: Sequence> {
    description: Option<Box<str>>,
    sequence: T,
}

impl<T: Sequence> Fasta<T> {
    pub fn parse(src: &str) -> TXResult<Self> {
        if let Ok((rem, comment)) = comment(src) {
            Ok(Self {
                description: Some(comment.into()),
                sequence: T::parse(rem)?,
            })
        } else {
            Ok(Self {
                description: None,
                sequence: T::parse(src)?,
            })
        }
    }

    pub fn description(&self) -> &Option<Box<str>> {
        &self.description
    }

    pub fn sequence(&self) -> &T {
        &self.sequence
    }
}

pub(crate) fn comment(src: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    delimited(one_of(">;"), take_until("\n"), newline).parse(src)
}

pub trait Sequence
where
    Self: Index<usize> + Sized,
{
    type Inner;

    fn parse(src: &str) -> Result<Self, TXError>;

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I);

    fn serialize(self) -> String;
}
