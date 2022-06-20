#![warn(missing_debug_implementations)]
#![allow(clippy::missing_errors_doc)]

pub mod err;
pub mod fasta;
pub mod fastq;
pub mod genomics;
pub mod gff;
pub mod proteomics;

pub(crate) mod util;
#[cfg(test)]
mod tests {}
