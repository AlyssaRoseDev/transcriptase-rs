use thiserror::Error;

pub type TXResult<T, E = TXError> = Result<T, E>;
#[derive(Debug, Error)]
pub enum TXError {
    #[error("Invalid codon: {0:?}")]
    InvalidCodon(String),
    #[error("IO Error: {0:?}")]
    BadFile(#[from] std::io::Error),
    #[error("Numeric conversion failed: {0:?}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("Formatting error: {0:?}")]
    FormatError(#[from] std::fmt::Error),
    #[error("Nom Error: {0}")]
    NomError(String),
    #[error("ParseInt Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl From<nom_supreme::error::ErrorTree<&str>> for TXError {
    fn from(src: nom_supreme::error::ErrorTree<&str>) -> Self {
        Self::NomError(src.to_string())
    }
}

type NomTreeErr<'a> = nom::Err<
    nom_supreme::error::GenericErrorTree<
        &'a str,
        &'a str,
        &'a str,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>;

impl From<NomTreeErr<'_>> for TXError {
    fn from(src: NomTreeErr) -> Self {
        Self::NomError(src.to_string())
    }
}

impl From<&NomTreeErr<'_>> for TXError {
    fn from(src: &NomTreeErr) -> Self {
        Self::NomError(src.to_string())
    }
}

impl From<&mut NomTreeErr<'_>> for TXError {
    fn from(src: &mut NomTreeErr) -> Self {
        Self::NomError(src.to_string())
    }
}
