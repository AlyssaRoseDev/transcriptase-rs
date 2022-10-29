use std::{
    fmt::Display,
    iter::FromIterator,
    ops::{Index, IndexMut},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::fasta::Sequence;

use self::amino::AminoAcid;

pub mod amino;

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

    const VALID_CHARS: &'static str = "ARNDCQEGHILKMFPSTWYVUO*";
}

impl FromIterator<AminoAcid> for Proteome {
    fn from_iter<T: IntoIterator<Item = AminoAcid>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(feature = "rayon")]
impl rayon::prelude::FromParallelIterator<AminoAcid> for Proteome {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = AminoAcid>,
    {
        Self(par_iter.into_par_iter().collect())
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
