use std::ops::Index;

use crate::{err::TXError, fasta::Sequence};

use self::amino::AminoAcid;

mod amino;

pub struct Proteome(Vec<AminoAcid>);

impl Index<usize> for Proteome {
    type Output = AminoAcid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Sequence for Proteome {
    type Inner = AminoAcid;

    fn parse(src: &str) -> Result<Self, TXError> {
        Ok(Self(
            src.lines()
                .flat_map(|line| line.chars().map(AminoAcid::try_from))
                .collect::<Result<Vec<AminoAcid>, TXError>>()?,
        ))
    }

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I) {
        self.0.extend(iter)
    }

    fn serialize(self) -> String {
        todo!()
    }
}
