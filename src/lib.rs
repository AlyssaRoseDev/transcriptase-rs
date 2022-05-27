// #![warn(clippy::pedantic, missing_debug_implementations, missing_docs)]
#![allow(clippy::missing_errors_doc)]

pub mod err;
pub mod fasta;
pub mod genomics;
pub mod gff;
pub mod proteomics;

#[cfg(test)]
mod tests {}
