use std::{ops::{Index, IndexMut}, fmt::Display};

use crate::{err::TXaseError, fasta::Sequence};

use self::amino::AminoAcid;

mod amino;

#[derive(Debug, Clone)]
pub struct Proteome(Vec<AminoAcid>);

impl Index<usize> for Proteome {
    type Output = AminoAcid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Proteome {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Sequence for Proteome {
    type Inner = AminoAcid;

    fn parse(src: &str) -> Result<Self, TXaseError> {
        Ok(Self(
            src.lines()
                .flat_map(|line| line.chars().map(AminoAcid::try_from))
                .collect::<Result<Vec<AminoAcid>, TXaseError>>()?,
        ))
    }

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    fn serialize(self) -> String {
        todo!()
    }
}

impl Display for Proteome {
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