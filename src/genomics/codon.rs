use crate::err::{TXError, TXResult};
use std::fmt::Display;
use std::{
    ops::{Deref, DerefMut},
    result::Result,
    str::FromStr,
};
/*
These hex representations allow for the representation of IUPAC extended Codons to match to the parallel
bitwise operations on the Nucleobases, e.g. Amino(M) to be represented as the bitwise OR of Adenine and
Cytosine. The special case is the NotX representations, which are the bitwise not of the nucleobase
followed by the bitwise AND of 0b11110000 to ensure the representation conforms to 4 bits per Codon.
This would be reversed (aka bitwise NOT of the mask) to read the Codon at the high end of each byte.
Codon, this is 2 per byte, or 0.5 KB per Kbp. This makes the Human Genome (approx. 6.2 Gbp) 3.1 GB
*/
///CODONS stores the ASCII encoding of the representative characters for each supported nucleobase
pub const DNACODONS: [u8; 16] = [
    b'0', b'A', b'C', b'M', b'G', b'R', b'S', b'V', b'T', b'W', b'Y', b'H', b'K', b'D', b'B', b'N',
];
pub const MASK: u8 = 0x0F;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum DNACodon {
    ZERO = 0x0,
    ADENINE = 0x1,
    CYTOSINE = 0x2,
    AMINO = 0x3,
    GUANINE = 0x4,
    PURINE = 0x5,
    STRONG = 0x6,
    NOTU = 0x7,
    THYMINE = 0x8,
    WEAK = 0x9,
    PYRIMIDINE = 0xA,
    NOTG = 0xB,
    KETO = 0xC,
    NOTC = 0xD,
    NOTA = 0xE,
    ANY = 0xF,
}

impl Display for DNACodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", DNACODONS[*self as usize])
    }
}

impl Default for DNACodon {
    fn default() -> Self {
        Self::ANY
    }
}

impl FromStr for DNACodon {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "0" => Ok(DNACodon::ZERO),
            "A" => Ok(DNACodon::ADENINE),
            "C" => Ok(DNACodon::CYTOSINE),
            "M" => Ok(DNACodon::AMINO),
            "G" => Ok(DNACodon::GUANINE),
            "R" => Ok(DNACodon::PURINE),
            "S" => Ok(DNACodon::STRONG),
            "V" => Ok(DNACodon::NOTU),
            "T" => Ok(DNACodon::THYMINE),
            "W" => Ok(DNACodon::WEAK),
            "Y" => Ok(DNACodon::PYRIMIDINE),
            "H" => Ok(DNACodon::NOTG),
            "K" => Ok(DNACodon::KETO),
            "D" => Ok(DNACodon::NOTC),
            "B" => Ok(DNACodon::NOTA),
            "N" => Ok(DNACodon::ANY),
            _ => Err(TXError::InvalidCodon(String::from(s))),
        }
    }
}

impl From<char> for DNACodon {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::ZERO,
            'A' => Self::ADENINE,
            'C' => Self::CYTOSINE,
            'M' => Self::AMINO,
            'G' => Self::GUANINE,
            'R' => Self::PURINE,
            'S' => Self::STRONG,
            'V' => Self::NOTU,
            'T' => Self::THYMINE,
            'W' => Self::WEAK,
            'Y' => Self::PYRIMIDINE,
            'H' => Self::NOTG,
            'K' => Self::KETO,
            'D' => Self::NOTC,
            'B' => Self::NOTA,
            'N' => Self::ANY,
            _ => Self::ZERO,
        }
    }
}

impl From<DNACodon> for char {
    fn from(c: DNACodon) -> Self {
        char::from(DNACODONS[c as usize])
    }
}

impl From<DNACodon> for u8 {
    fn from(codon: DNACodon) -> Self {
        codon as u8
    }
}

pub const RNACODONS: [u8; 16] = [
    b'0', b'A', b'C', b'M', b'G', b'R', b'S', b'V', b'U', b'W', b'Y', b'H', b'K', b'D', b'B', b'N',
];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RNACodon {
    ZERO = 0x0,
    ADENINE = 0x1,
    CYTOSINE = 0x2,
    AMINO = 0x3,
    GUANINE = 0x4,
    PURINE = 0x5,
    STRONG = 0x6,
    NOTU = 0x7,
    URACIL = 0x8,
    WEAK = 0x9,
    PYRIMIDINE = 0xA,
    NOTG = 0xB,
    KETO = 0xC,
    NOTC = 0xD,
    NOTA = 0xE,
    ANY = 0xF,
}

impl Display for RNACodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", RNACODONS[*self as usize])
    }
}

impl Default for RNACodon {
    fn default() -> Self {
        Self::ANY
    }
}

impl FromStr for RNACodon {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "0" => Ok(RNACodon::ZERO),
            "A" => Ok(RNACodon::ADENINE),
            "C" => Ok(RNACodon::CYTOSINE),
            "M" => Ok(RNACodon::AMINO),
            "G" => Ok(RNACodon::GUANINE),
            "R" => Ok(RNACodon::PURINE),
            "S" => Ok(RNACodon::STRONG),
            "V" => Ok(RNACodon::NOTU),
            "U" => Ok(RNACodon::URACIL),
            "W" => Ok(RNACodon::WEAK),
            "Y" => Ok(RNACodon::PYRIMIDINE),
            "H" => Ok(RNACodon::NOTG),
            "K" => Ok(RNACodon::KETO),
            "D" => Ok(RNACodon::NOTC),
            "B" => Ok(RNACodon::NOTA),
            "N" => Ok(RNACodon::ANY),
            _ => Err(TXError::InvalidCodon(String::from(s))),
        }
    }
}

impl From<char> for RNACodon {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::ZERO,
            'A' => Self::ADENINE,
            'C' => Self::CYTOSINE,
            'M' => Self::AMINO,
            'G' => Self::GUANINE,
            'R' => Self::PURINE,
            'S' => Self::STRONG,
            'V' => Self::NOTU,
            'U' => Self::URACIL,
            'W' => Self::WEAK,
            'Y' => Self::PYRIMIDINE,
            'H' => Self::NOTG,
            'K' => Self::KETO,
            'D' => Self::NOTC,
            'B' => Self::NOTA,
            'N' => Self::ANY,
            _ => Self::ZERO,
        }
    }
}

impl From<RNACodon> for char {
    fn from(c: RNACodon) -> Self {
        char::from(RNACODONS[c as usize])
    }
}

impl From<RNACodon> for u8 {
    fn from(codon: RNACodon) -> Self {
        codon as u8
    }
}
