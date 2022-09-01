use crate::err::{TXaseError, TXaseResult};
use either::Either;
use nom::{
    bytes::complete::take_until,
    character::complete::{newline, one_of},
    sequence::delimited,
    Parser,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{ops::Index, str::FromStr};

/// [`Fasta`] is a simple text-based format for genomic and proteomic sequences that stores an optional
/// description and a sequence of [`RNA`](crate::genomics::nucleotide::RNA), [`DNA`](crate::genomics::nucleotide::DNA), or [`Amino Acids`](crate::proteomics::amino::AminoAcid).
#[derive(Debug, Clone)]
pub struct Fasta<T: Sequence> {
    /// The description given in the inital comment line
    pub description: Option<Box<str>>,
    /// The genomic or proteomic sequence
    pub sequence: T,
}

impl<T: Sequence> Fasta<T> {
    /// Parses a string slice as a [`Fasta`] formatted document.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Sequence`]
    /// parse implementation returns an error
    #[cfg(not(feature = "rayon"))]
    pub fn parse(src: &str) -> TXaseResult<Either<Self, Vec<Self>>> {
        let mut seqs: Vec<Fasta<T>> = crate::util::Split2Iter::new(b'>', b';', src)
            .map(|seq| {
                let (d, s) = comment(seq);
                Ok(Self {
                    description: d.map(Into::into),
                    sequence: T::from_str(s)?,
                })
            })
            .collect::<TXaseResult<Vec<Fasta<T>>>>()?;
        if seqs.len() == 1 {
            Ok(Either::Left(seqs.pop().expect("len is at least 1")))
        } else {
            Ok(Either::Right(seqs))
        }
    }
}
impl<T: Sequence + Send> Fasta<T> {
    /// Parses a string slice as a [`Fasta`] formatted document.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Sequence`]
    /// parse implementation returns an error
    #[cfg(feature = "rayon")]
    pub fn parse(src: &str) -> TXaseResult<Either<Self, Vec<Self>>> {
        let mut seqs: Vec<Fasta<T>> = crate::util::Split2Iter::new(b'>', b';', src)
            .into_par_iter()
            .map(|seq| {
                let (d, s) = comment(seq);
                Ok(Self {
                    description: d.map(Into::into),
                    sequence: T::from_str(s)?,
                })
            })
            .collect::<TXaseResult<Vec<Fasta<T>>>>()?;
        if seqs.len() == 1 {
            Ok(Either::Left(seqs.pop().expect("len is at least 1")))
        } else {
            Ok(Either::Right(seqs))
        }
    }
}

pub(crate) fn comment(src: &str) -> (Option<&str>, &str) {
    delimited::<_, _, _, _, (), _, _, _>(one_of(">;"), take_until("\n"), newline)
        .parse(src)
        .ok()
        .map(|(comment, seq)| (Some(comment), seq))
        .unwrap_or_else(|| (None, src))
}

/// A sequence that can be:
/// - Parsed from a text format
/// - Serialized to a text format
/// - Serialized to a raw binary representation
pub trait Sequence
where
    Self: Index<usize> + Extend<Self::Inner> + FromStr<Err = TXaseError> + Sized,
{
    /// The type of each member of the sequence
    type Inner: TryFrom<char>;

    /// Serialize this `Sequence` to a text format
    fn serialize(&self) -> String;

    /// Serialize this `Sequence` to a raw binary stream
    fn serialize_bytes(&self) -> &[u8];
}
