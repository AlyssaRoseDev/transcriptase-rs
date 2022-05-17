pub mod codon;
pub mod genome;

pub(crate) mod prelude {
    pub use crate::genomics::{codon::*, genome::*};
}
