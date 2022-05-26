use super::{
    nucleotide::DNA,
    prelude::{DNA_CODONS, RNA},
};
use crate::{err::TXError, fasta::Sequence};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct DnaSeq(Vec<DNA>);

impl Sequence for DnaSeq {
    type Inner = DNA;

    fn parse(src: &str) -> Result<Self, TXError> {
        Ok(Self(
            src.lines()
                .flat_map(|line| line.chars().map(DNA::try_from))
                .collect::<Result<Vec<DNA>, TXError>>()?,
        ))
    }

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    fn serialize(self) -> String {
        self.to_string()
    }
}

impl Index<usize> for DnaSeq {
    type Output = DNA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for DnaSeq {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for DnaSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::with_capacity(60);
        for chunk in self.0.chunks(60) {
            line.extend(chunk.iter().map(|c| DNA_CODONS[*c as usize] as char));
            writeln!(f, "{line}")?;
            line.clear();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RnaSeq(Vec<RNA>);

impl Sequence for RnaSeq {
    type Inner = RNA;

    fn parse(src: &str) -> Result<Self, TXError> {
        Ok(Self(
            src.lines()
                .flat_map(|line| line.chars().map(RNA::try_from))
                .collect::<Result<Vec<RNA>, TXError>>()?,
        ))
    }

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    fn serialize(self) -> String {
        self.to_string()
    }
}

impl Index<usize> for RnaSeq {
    type Output = RNA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for RnaSeq {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for RnaSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::with_capacity(60);
        for chunk in self.0.chunks(60) {
            line.extend(chunk.iter().map(char::from));
            writeln!(f, "{line}")?;
            line.clear();
        }
        Ok(())
    }
}
