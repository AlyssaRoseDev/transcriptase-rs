use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use rayon::prelude::*;

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

    fn serialize(&self) -> String {
        todo!()
    }

    fn serialize_bytes(&self) -> &[u8] {
        todo!()
    }
}

#[cfg(feature = "rayon")]
impl FromStr for Proteome {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.par_lines()
                .flat_map(|line| line.par_chars().map(AminoAcid::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(not(feature = "rayon"))]
impl FromStr for Proteome {
    type Err = TXaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .flat_map(|line| line.chars().map(AminoAcid::try_from))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Extend<AminoAcid> for Proteome {
    fn extend<T: IntoIterator<Item = AminoAcid>>(&mut self, iter: T) {
        self.0.extend(iter)
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
