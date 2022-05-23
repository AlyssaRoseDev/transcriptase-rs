use thiserror::Error;

pub type TXResult<T, E = TXError> = Result<T, E>;
#[derive(Debug, Error)]
pub enum TXError {
    #[error("Invalid nucleotide: {0:?}")]
    InvalidNucleotide(String),
    #[error("Invalid amino acid: {0:?}")]
    InvalidCodon(String),
    #[error("IO Error: {0:?}")]
    BadFile(#[from] std::io::Error),
    #[error("Numeric conversion failed: {0:?}")]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error("Formatting error: {0:?}")]
    Formatting(#[from] std::fmt::Error),
    #[error("Nom Error: {0}")]
    NomParsing(String),
    #[error("ParseInt Error: {0}")]
    StdIntParse(#[from] std::num::ParseIntError),
    #[error("Invalid Attribute: {0}")]
    InvalidAttribute(String),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput(),
    #[error("Encountered Duplicate Id Attribute in GFF Entry")]
    DuplicateGFFEntryID(),
}

impl From<nom_supreme::error::ErrorTree<&str>> for TXError {
    fn from(src: nom_supreme::error::ErrorTree<&str>) -> Self {
        Self::NomParsing(src.to_string())
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
        Self::NomParsing(src.to_string())
    }
}