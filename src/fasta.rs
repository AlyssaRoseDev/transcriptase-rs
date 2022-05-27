use crate::err::{TXaseError, TXaseResult};
use nom::{
    bytes::complete::take_until,
    character::complete::{newline, one_of},
    sequence::delimited,
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use std::ops::Index;

// #[derive(Debug, Clone)]
// pub struct _Fasta<T: Sequence> {
//     description: Option<Box<str>>,
//     sequence: T,
// }

/// [`Fasta`] is a simple text-based format for genomic and proteomic sequences that stores an optional
/// description and a sequence of [`RNA`](crate::genomics::nucleotide::RNA), [`DNA`](crate::genomics::nucleotide::DNA), or [`Amino Acids`](crate::proteomics::amino::AminoAcid).
#[derive(Debug, Clone)]
pub enum Fasta<T: Sequence> {
    /// This is a single sequence file
    Single {
        description: Box<str>,
        sequence: T
    },
    /// This is a multi-sequence file. The descriptions and sequences are guaranteed to be in the same order
    Multi {
        descriptions: Vec<Box<str>>,
        sequences: Vec<T> 
    }
}

impl<T: Sequence> Fasta<T> {
    /// Parses a string slice as a [`Fasta`] formatted document.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Sequence`]
    /// implementation returns an error
    #[allow(unused_assignments)]
    pub fn parse(src: &str) -> TXaseResult<Self> {
        let mut iter = memchr::memchr2_iter(b'>', b';', src.as_bytes()).peekable();
        let first = true;
        loop {
            let mut from = src;
            // May I be forgiven for my sins against the heap and generics
            let mut descriptions: Option<Vec<Box<str>>> = None;
            let mut sequences: Option<Vec<T>> = None;
            match (iter.next(), iter.peek()) {
                (None, None) => {
                    if first {
                        return Err(TXaseError::InternalParseFailure(String::from("Empty string passed to Fasta::parse()")));
                    } else {
                        return Ok(Self::Multi {
                            descriptions: descriptions.take().ok_or_else(|| TXaseError::InternalParseFailure(String::from("Fasta::parse() iterated more than once and descs was not populated")))?,
                            sequences: sequences.take().ok_or_else(|| TXaseError::InternalParseFailure(String::from("Fasta::parse() iterated more than once and seqs was not populated ")))?,
                        })
                    }
                },
                (None, Some(_)) => unreachable!(), // If next is None, peek cannot be Some
                (Some(idx), None) => { // This is the last, or only, sequence
                    if first {
                        return comment(from)
                        .map_err(TXaseError::from)
                        .and_then(|(comment, seq)| {
                            Ok(Self::Single { description: comment.into(), sequence: T::parse(seq)? })
                        });
                    } else {
                        match (descriptions, sequences) {
                            (None, None) => { // I don't *think* this is a reachable block, but we will see
                                let mut descs_vec = Vec::new();
                                let mut seqs_vec = Vec::new();
                                let (seq, comment) = comment(&from[idx..])?;
                                descs_vec.push(comment.into());
                                seqs_vec.push(T::parse(seq)?);
                                descriptions = Some(descs_vec);
                                sequences = Some(seqs_vec);
                            },
                            (Some(ref mut descs), Some(ref mut seqs)) => {
                                let (seq, comment) = comment(&from[idx..])?;
                                descs.push(comment.into());
                                seqs.push(T::parse(seq)?);
                            },
                            _ => unreachable!(),
                        }
                    }
                },
                // There is additional sequences
                (Some(start), Some(&end)) => {
                    match (descriptions, sequences) {
                        (None, None) => {
                                let mut descs_vec = Vec::new();
                                let mut seqs_vec = Vec::new();
                                let (seq, comment) = comment(&from[start..end])?;
                                descs_vec.push(comment.into());
                                seqs_vec.push(T::parse(seq)?);
                                descriptions = Some(descs_vec);
                                sequences = Some(seqs_vec);
                        },
                        (Some(ref mut descs), Some(ref mut seqs)) => {
                            let (seq, comment) = comment(&from[start..end])?;
                            descs.push(comment.into());
                            seqs.push(T::parse(seq)?);
                        },
                        _ => unreachable!(),
                    }
                    from = &from[end..];
                },
            }
        }
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

    fn parse(src: &str) -> Result<Self, TXaseError>;

    fn extend<I: IntoIterator<Item = Self::Inner>>(&mut self, iter: I);

    fn serialize(self) -> String;
}
