pub mod genome;
pub mod nucleotide;

pub(crate) mod prelude {
    pub use crate::genomics::{
        genome::{DnaSeq, RnaSeq},
        nucleotide::{DNA, RNA},
    };
}
