pub mod nucleotide;
pub mod genome;

pub(crate) mod prelude {
    pub use crate::genomics::{nucleotide::*, genome::*};
}
