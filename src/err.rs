use thiserror::Error;

/// Type alias for a Result with the [`TXaseError`] as it's default error type
pub type TXaseResult<T, E = TXaseError> = Result<T, E>;

/// Collection of the errors that can be returned from the public API
#[derive(Debug, Error)]
pub enum TXaseError {
    /// An invalid nucleotide was found while converting text to a
    /// [`DNA`](crate::genomics::genome::DNA) or [`RNA`](crate::genomics::genome::RNA)
    #[error("Invalid nucleotide: {0:?}")]
    InvalidNucleotide(String),
    /// An invalid codon was found while translating from a
    /// [`DNA`](crate::genomics::genome::DNA) or [`RNA`](crate::genomics::genome::RNA)
    /// to an [`AminoAcid`](crate::proteomics::AminoAcid)
    #[error("Invalid amino acid: {0:?}")]
    InvalidCodon(String),
    /// An [`IoError`](std::io::Error)
    #[error("IO Error: {0:?}")]
    StdIo(#[from] std::io::Error),
    /// A [`FmtError`](std::fmt::Error)
    #[error("Formatting error: {0:?}")]
    StdFmt(#[from] std::fmt::Error),
    /// An error that occurs during Nom text parsing
    #[error("Nom Error: {0}")]
    NomParsing(String),
    /// A [`ParseIntError`](std::num::ParseIntError)
    #[error("ParseInt Error: {0}")]
    StdIntParse(#[from] std::num::ParseIntError),
    /// An invalid [`Attribute`](crate::gff::attr::Attribute) kind,
    /// as attributes that start with uppercase letters are reserved
    #[error("Invalid Attribute: {0}")]
    InvalidAttribute(String),
    /// A duplicate [`Id`](crate::gff::attr::Id) was found while parsing an entry
    #[error("Encountered Duplicate Id Attribute in GFF Entry")]
    DuplicateGFFEntryID(),
    /// A [`Utf8Error`](std::str::Utf8Error)
    #[error("{0}")]
    StdUTF8(#[from] std::str::Utf8Error),
    /// A parsing error that occured in the library itself
    #[error("{0}")]
    InternalParseFailure(String),
}

type NomTreeErr<'a> = nom::Err<
    nom_supreme::error::GenericErrorTree<
        &'a str,
        &'a str,
        &'a str,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>;

impl From<NomTreeErr<'_>> for TXaseError {
    fn from(src: NomTreeErr) -> Self {
        Self::NomParsing(src.to_string())
    }
}
