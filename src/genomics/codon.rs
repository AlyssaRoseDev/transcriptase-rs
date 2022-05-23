use crate::err::TXError;
use std::fmt::Display;
use std::{result::Result, str::FromStr};
/*
These hex representations allow for the representation of IUPAC extended Codons to match to the parallel
bitwise operations on the Nucleobases, e.g. Amino(M) to be represented as the bitwise OR of Adenine and
Cytosine. The special case is the NotX representations, which are the bitwise not of the nucleobase
followed by the bitwise AND of 0b11110000 to ensure the representation conforms to 4 bits per Codon.
This would be reversed (aka bitwise NOT of the mask) to read the Codon at the high end of each byte.
Codon, this is 2 per byte, or 0.5 KB per Kbp. This makes the Human Genome (approx. 6.2 Gbp) 3.1 GB
*/
///CODONS stores the ASCII encoding of the representative characters for each supported nucleobase
pub const DNA_CODONS: [u8; 16] = [
    b'0', b'A', b'C', b'M', b'G', b'R', b'S', b'V', b'T', b'W', b'Y', b'H', b'K', b'D', b'B', b'N',
];
pub const MASK: u8 = 0x0F;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum DNA {
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

impl Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", DNA_CODONS[*self as usize])
    }
}

impl Default for DNA {
    fn default() -> Self {
        Self::ANY
    }
}

impl FromStr for DNA {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "0" => Ok(DNA::ZERO),
            "A" => Ok(DNA::ADENINE),
            "C" => Ok(DNA::CYTOSINE),
            "M" => Ok(DNA::AMINO),
            "G" => Ok(DNA::GUANINE),
            "R" => Ok(DNA::PURINE),
            "S" => Ok(DNA::STRONG),
            "V" => Ok(DNA::NOTU),
            "T" => Ok(DNA::THYMINE),
            "W" => Ok(DNA::WEAK),
            "Y" => Ok(DNA::PYRIMIDINE),
            "H" => Ok(DNA::NOTG),
            "K" => Ok(DNA::KETO),
            "D" => Ok(DNA::NOTC),
            "B" => Ok(DNA::NOTA),
            "N" => Ok(DNA::ANY),
            _ => Err(TXError::InvalidNucleotide(String::from(s))),
        }
    }
}

impl TryFrom<char> for DNA {
    type Error = TXError;

    fn try_from(value: char) -> Result<Self, TXError> {
        Ok(match value {
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
            _ => return Err(TXError::InvalidNucleotide(String::from(value))),
        })
    }
}

impl TryFrom<u8> for DNA {
    type Error = TXError;

    fn try_from(value: u8) -> Result<Self, TXError> {
        Ok(match value {
            0x0 => Self::ZERO,
            0x1 => Self::ADENINE,
            0x2 => Self::CYTOSINE,
            0x3 => Self::AMINO,
            0x4 => Self::GUANINE,
            0x5 => Self::PURINE,
            0x6 => Self::STRONG,
            0x7 => Self::NOTU,
            0x8 => Self::THYMINE,
            0x9 => Self::WEAK,
            0xA => Self::PYRIMIDINE,
            0xB => Self::NOTG,
            0xC => Self::KETO,
            0xD => Self::NOTC,
            0xE => Self::NOTA,
            0xF => Self::ANY,
            _ => return Err(TXError::InvalidNucleotide(format!("{value}"))),
        })
    }
}

impl From<DNA> for char {
    fn from(c: DNA) -> Self {
        DNA_CODONS[c as usize] as char
    }
}

impl From<DNA> for u8 {
    fn from(codon: DNA) -> Self {
        codon as u8
    }
}

pub const RNA_CODONS: [u8; 16] = [
    b'0', b'A', b'C', b'M', b'G', b'R', b'S', b'V', b'U', b'W', b'Y', b'H', b'K', b'D', b'B', b'N',
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum RNA {
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

impl Display for RNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", RNA_CODONS[*self as usize])
    }
}

impl Default for RNA {
    fn default() -> Self {
        Self::ANY
    }
}

impl FromStr for RNA {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "0" => Ok(RNA::ZERO),
            "A" => Ok(RNA::ADENINE),
            "C" => Ok(RNA::CYTOSINE),
            "M" => Ok(RNA::AMINO),
            "G" => Ok(RNA::GUANINE),
            "R" => Ok(RNA::PURINE),
            "S" => Ok(RNA::STRONG),
            "V" => Ok(RNA::NOTU),
            "U" => Ok(RNA::URACIL),
            "W" => Ok(RNA::WEAK),
            "Y" => Ok(RNA::PYRIMIDINE),
            "H" => Ok(RNA::NOTG),
            "K" => Ok(RNA::KETO),
            "D" => Ok(RNA::NOTC),
            "B" => Ok(RNA::NOTA),
            "N" => Ok(RNA::ANY),
            _ => Err(TXError::InvalidNucleotide(String::from(s))),
        }
    }
}

impl TryFrom<char> for RNA {
    type Error = TXError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
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
            _ => return Err(TXError::InvalidNucleotide(String::from(value))),
        })
    }
}

impl TryFrom<u8> for RNA {
    type Error = TXError;

    fn try_from(value: u8) -> Result<Self, TXError> {
        Ok(match value {
            0x0 => Self::ZERO,
            0x1 => Self::ADENINE,
            0x2 => Self::CYTOSINE,
            0x3 => Self::AMINO,
            0x4 => Self::GUANINE,
            0x5 => Self::PURINE,
            0x6 => Self::STRONG,
            0x7 => Self::NOTU,
            0x8 => Self::URACIL,
            0x9 => Self::WEAK,
            0xA => Self::PYRIMIDINE,
            0xB => Self::NOTG,
            0xC => Self::KETO,
            0xD => Self::NOTC,
            0xE => Self::NOTA,
            0xF => Self::ANY,
            _ => return Err(TXError::InvalidNucleotide(format!("{value}"))),
        })
    }
}
