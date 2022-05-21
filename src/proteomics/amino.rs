use std::str::FromStr;

use crate::err::TXError;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    Aspartate,
    Cysteine,
    Glutamine,
    Glutamate,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptonphan,
    Tyrosine,
    Valine,
    Selenocysteine,
    Pyrrolysine,
}

impl AminoAcid {
    const ABBREV: [&'static str; 22] = [
        "Ala", "Arg", "Asn", "Asp", "Cys", "Gln", "Glu", "Gly", "His", "Ile", "Leu", "Lys", "Met",
        "Phe", "Pro", "Ser", "Thr", "Trp", "Tyr", "Val", "Sec", "Pyl",
    ];

    const SHORT: [char; 22] = [
        'A', 'R', 'N', 'D', 'C', 'Q', 'E', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T', 'W',
        'Y', 'V', 'U', 'O',
    ];

    const LONG: [&'static str; 22] = [
        "Alanine",
        "Arginine",
        "Asparagine",
        "Aspartate",
        "Cysteine",
        "Glutamine",
        "Glutamate",
        "Glycine",
        "Histidine",
        "Isoleucine",
        "Leucine",
        "Lysine",
        "Methionine",
        "Phenylalanine",
        "Proline",
        "Serine",
        "Threonine",
        "Tryptonphan",
        "Tyrosine",
        "Valine",
        "Selenocysteine",
        "Pyrrolysine",
    ];

    #[must_use]
    pub fn abbreviation(&self) -> &'static str {
        Self::ABBREV[*self as usize]
    }

    #[must_use]
    pub fn short(&self) -> char {
        Self::SHORT[*self as usize]
    }

    #[must_use]
    pub fn long(&self) -> &'static str {
        Self::LONG[*self as usize]
    }
}

impl FromStr for AminoAcid {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Alanine" | "Ala" | "A" => Self::Alanine,
            "Arginine" | "Arg" | "R" => Self::Arginine,
            "Asparagine" | "Asn" | "N" => Self::Asparagine,
            "Aspartate" | "Asp" | "D" => Self::Aspartate,
            "Cysteine" | "Cys" | "C" => Self::Cysteine,
            "Glutamine" | "Gln" | "Q" => Self::Glutamine,
            "Glutamate" | "Glu" | "E" => Self::Glutamate,
            "Glycine" | "Gly" | "G" => Self::Glycine,
            "Histidine" | "His" | "H" => Self::Histidine,
            "Isoleucine" | "Ile" | "I" => Self::Isoleucine,
            "Leucine" | "Leu" | "L" => Self::Leucine,
            "Lysine" | "Lys" | "K" => Self::Lysine,
            "Methionine" | "Met" | "M" => Self::Methionine,
            "Phenylalanine" | "Phe" | "F" => Self::Phenylalanine,
            "Proline" | "Pro" | "P" => Self::Proline,
            "Serine" | "Ser" | "S" => Self::Serine,
            "Threonine" | "Thr" | "T" => Self::Threonine,
            "Tryptonphan" | "Trp" | "W" => Self::Tryptonphan,
            "Tyrosine" | "Tyr" | "Y" => Self::Tyrosine,
            "Valine" | "Val" | "V" => Self::Valine,
            "Selenocysteine" | "Sec" | "U" => Self::Selenocysteine,
            "Pyrrolysine" | "Pyl" | "O" => Self::Pyrrolysine,
            _ => return Err(TXError::InvalidAminoAcid(String::from(s))),
        })
    }
}

impl TryFrom<char> for AminoAcid {
    type Error = TXError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Alanine,
            'R' => Self::Arginine,
            'N' => Self::Asparagine,
            'D' => Self::Aspartate,
            'C' => Self::Cysteine,
            'Q' => Self::Glutamine,
            'E' => Self::Glutamate,
            'G' => Self::Glycine,
            'H' => Self::Histidine,
            'I' => Self::Isoleucine,
            'L' => Self::Leucine,
            'K' => Self::Lysine,
            'M' => Self::Methionine,
            'F' => Self::Phenylalanine,
            'P' => Self::Proline,
            'S' => Self::Serine,
            'T' => Self::Threonine,
            'W' => Self::Tryptonphan,
            'Y' => Self::Tyrosine,
            'V' => Self::Valine,
            'U' => Self::Selenocysteine,
            'O' => Self::Pyrrolysine,
            val => return Err(TXError::InvalidAminoAcid(val.to_string())),
        })
    }
}
