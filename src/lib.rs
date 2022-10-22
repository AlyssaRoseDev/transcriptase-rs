#![warn(missing_debug_implementations)]

use nom::{error::VerboseError, IResult};

pub mod err;
pub mod fasta;
pub mod fastq;
pub mod genomics;
pub mod gff;
pub mod proteomics;

pub(crate) type NomResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

#[cfg(test)]
mod tests {}
