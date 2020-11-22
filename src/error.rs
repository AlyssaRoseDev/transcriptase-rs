use std::{error::Error, fmt::Display};

pub type TxaseResult<T, E = TxaseErr> = Result<T, E>;
#[derive(Debug)]
pub enum TxaseErr {
    InvalidCodon(String),
    BadFile(std::io::Error),
}

impl Display for TxaseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxaseErr::InvalidCodon(s) => write!(f, "Error: Invalid Codon {:?}", s),
            TxaseErr::BadFile(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for TxaseErr {
    fn from(e: std::io::Error) -> Self {
        return Self::BadFile(e);
    }
}

impl Error for TxaseErr {}
