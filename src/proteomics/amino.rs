use std::str::FromStr;

use crate::err::{TXaseError, TXaseResult};

use self::translation::{DNA_TRANSLATION_TABLE, RNA_TRANSLATION_TABLE};
mod translation;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(clippy::module_name_repetitions)]
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
    Stop,
}

impl AminoAcid {
    const ABBREV: [&'static str; 23] = [
        "Ala", "Arg", "Asn", "Asp", "Cys", "Gln", "Glu", "Gly", "His", "Ile", "Leu", "Lys", "Met",
        "Phe", "Pro", "Ser", "Thr", "Trp", "Tyr", "Val", "Sec", "Pyl", "Ter",
    ];

    const SHORT: [char; 23] = [
        'A', 'R', 'N', 'D', 'C', 'Q', 'E', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T', 'W',
        'Y', 'V', 'U', 'O', '*',
    ];

    const LONG: [&'static str; 23] = [
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
        "Tryptophan",
        "Tyrosine",
        "Valine",
        "Selenocysteine",
        "Pyrrolysine",
        "Translation Stop",
    ];

    #[must_use]
    pub fn abbreviation(self) -> &'static str {
        Self::ABBREV[self as usize]
    }

    #[must_use]
    pub fn short(self) -> char {
        Self::SHORT[self as usize]
    }

    #[must_use]
    pub fn long(self) -> &'static str {
        Self::LONG[self as usize]
    }

    pub fn translate_rna(codon: &str) -> TXaseResult<Self> {
        RNA_TRANSLATION_TABLE
            .get(codon)
            .copied()
            .ok_or_else(|| TXaseError::InvalidCodon(codon.to_string()))
    }

    pub fn translate_dna(codon: &str) -> TXaseResult<Self> {
        DNA_TRANSLATION_TABLE
            .get(codon)
            .copied()
            .ok_or_else(|| TXaseError::InvalidCodon(codon.to_string()))
    }
}

impl FromStr for AminoAcid {
    type Err = TXaseError;

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
            "Tryptophan" | "Trp" | "W" => Self::Tryptonphan,
            "Tyrosine" | "Tyr" | "Y" => Self::Tyrosine,
            "Valine" | "Val" | "V" => Self::Valine,
            "Selenocysteine" | "Sec" | "U" => Self::Selenocysteine,
            "Pyrrolysine" | "Pyl" | "O" => Self::Pyrrolysine,
            "Amber" | "Ochre" | "Umber" | "Opal" | "Ter" | "*" => Self::Stop,
            _ => {
                return Err(TXaseError::InternalParseFailure(format!(
                    "Failed to parse {s} as an Amino Acid"
                )))
            }
        })
    }
}

impl TryFrom<char> for AminoAcid {
    type Error = TXaseError;

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
            '*' => Self::Stop,
            val => {
                return Err(TXaseError::InternalParseFailure(format!(
                    "Failed to parse {val} as an Amino Acid"
                )))
            }
        })
    }
}

impl From<AminoAcid> for char {
    fn from(aa: AminoAcid) -> Self {
        aa.short()
    }
}

impl From<&AminoAcid> for char {
    fn from(aa: &AminoAcid) -> Self {
        aa.short()
    }
}

impl From<&mut AminoAcid> for char {
    fn from(aa: &mut AminoAcid) -> Self {
        aa.short()
    }
}
