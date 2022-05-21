use super::{codon::DNA, prelude::DNACODONS};
use crate::{err::TXError, fasta::Sequence};
use std::{fmt::Display, ops::Index};

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
        self.0.extend(iter)
    }

    fn serialize(self) -> String {
        todo!()
    }
}

impl Index<usize> for DnaSeq {
    type Output = DNA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Display for DnaSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::with_capacity(60);
        self.0
            .chunks(60)
            .map(|chunk| {
                line.extend(chunk.iter().map(|c| DNACODONS[*c as usize] as char));
                let ret = writeln!(f, "{line}");
                line.clear();
                ret
            })
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        Ok(())
    }
}
