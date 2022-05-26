#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod err;
pub mod fasta;
pub mod genomics;
pub mod gff;
pub mod proteomics;

#[cfg(test)]
mod tests {}
