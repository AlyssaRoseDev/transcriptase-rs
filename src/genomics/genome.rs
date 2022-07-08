use super::nucleotide::{DNA, RNA};
use crate::{err::TXaseError, fasta::Sequence};
use rayon::prelude::*;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

/// A sequence of [`DNA`] nucleotides
#[derive(Debug, Clone)]
pub struct DnaSeq(Vec<DNA>);

impl Sequence for DnaSeq {
    type Inner = DNA;

    fn serialize(&self) -> String {
        todo!()
    }

    fn serialize_bytes(&self) -> &[u8] {
        todo!()
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

#[cfg(feature = "rayon")]
impl FromStr for DnaSeq {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.par_lines()
                .flat_map(|line| line.par_chars().map(DNA::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(not(feature = "rayon"))]
impl FromStr for DnaSeq {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .flat_map(|line| line.chars().map(DNA::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Extend<DNA> for DnaSeq {
    fn extend<T: IntoIterator<Item = DNA>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl Display for DnaSeq {
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

/// A sequence of [`RNA`] nucleotides
#[derive(Debug)]
pub struct RnaSeq(Vec<RNA>);

impl Sequence for RnaSeq {
    type Inner = RNA;

    fn serialize(&self) -> String {
        todo!()
    }

    fn serialize_bytes(&self) -> &[u8] {
        todo!()
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

#[cfg(feature = "rayon")]
impl FromStr for RnaSeq {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.par_lines()
                .flat_map(|line| line.par_chars().map(RNA::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(not(feature = "rayon"))]
impl FromStr for RnaSeq {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .flat_map(|line| line.chars().map(RNA::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Extend<RNA> for RnaSeq {
    fn extend<T: IntoIterator<Item = RNA>>(&mut self, iter: T) {
        self.0.extend(iter);
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
