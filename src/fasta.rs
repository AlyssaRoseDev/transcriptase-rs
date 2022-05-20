use std::iter;
use std::ops::Index;

use crate::err::TXResult;

mod parsers;

pub struct Fasta<T: Sequence> {
    description: Option<Box<str>>,
    sequence: T,
}

impl<T: Sequence> Fasta<T> {
    pub fn parse_dna(src: &str) -> TXResult<Self> {
        todo!()
    }

    pub fn parse_proteome(src: &str) -> TXResult<Self> {
        todo!()
    }
}

pub trait Sequence where Self: Index<usize> + Sized {
    type ParseError;
    type Inner;

    fn parse(src: &str) -> Result<Self, Self::ParseError>;

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I);

    fn serialize(self) -> String;
}