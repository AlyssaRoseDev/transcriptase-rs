use crate::error::TxaseErr;
use std::fmt::{Display, Error};
use std::{
    ops::{Deref, DerefMut},
    result::Result,
    str::FromStr,
};
/*
These hex representations allow for the representation of IUPAC extended Codons to match to the parallel
bitwise operations on the Nucleobases, e.g. Amino(M) to be represented as the bitwise OR of Adenine and
Cytosine. The special case is the NotX representations, which are the bitwise not of the nucleobase
followed by the bitwise AND of 0b11110000 to ensure the representation conforms to 4 bits per ExtCodon.
This would be reversed (aka bitwise NOT of the mask) to read the ExtCodon at the high end of each byte. At bits per
ExtCodon, this is 2 per byte, or 0.5 KB per Kbp. This makes the Human Genome (approx. 6.2 Gbp) 3.1 GB
*/

pub const CODONS: [char; 16] = [
    '0', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'T', 'W', 'Y', 'H', 'K', 'D', 'B', 'N',
];
pub const MASK_LOW: u8 = 0x0F;
pub const MASK_HIGH: u8 = 0xF0;
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Codon {
    ZERO = 0x0,
    ADENINE = 0x1,
    CYTOSINE = 0x2,
    AMINO = 0x3,
    GUANINE = 0x4,
    PURINE = 0x5,
    STRONG = 0x6,
    NotU = 0x7,
    THYMINE = 0x8,
    WEAK = 0x9,
    PYRIMIDINE = 0xA,
    NotG = 0xB,
    KETO = 0xC,
    NotC = 0xD,
    NotA = 0xE,
    ANY = 0xF,
}

impl Display for Codon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", CODONS[*self as usize])
    }
}

impl Default for Codon {
    fn default() -> Self {
        Self::ANY
    }
}

impl FromStr for Codon {
    type Err = TxaseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "0" => Ok(Codon::ZERO),
            "A" => Ok(Codon::ADENINE),
            "C" => Ok(Codon::CYTOSINE),
            "M" => Ok(Codon::AMINO),
            "G" => Ok(Codon::GUANINE),
            "R" => Ok(Codon::PURINE),
            "S" => Ok(Codon::STRONG),
            "V" => Ok(Codon::NotU),
            "T" => Ok(Codon::THYMINE),
            "W" => Ok(Codon::WEAK),
            "Y" => Ok(Codon::PYRIMIDINE),
            "H" => Ok(Codon::NotG),
            "K" => Ok(Codon::KETO),
            "D" => Ok(Codon::NotC),
            "B" => Ok(Codon::NotA),
            "N" => Ok(Codon::ANY),
            _ => Err(TxaseErr::InvalidCodon(format!(
                "Codon::from_str could not parse {}",
                String::from(s)
            ))),
        }
    }
}

impl From<char> for Codon {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::ZERO,
            'A' => Self::ADENINE,
            'C' => Self::CYTOSINE,
            'M' => Self::AMINO,
            'G' => Self::GUANINE,
            'R' => Self::PURINE,
            'S' => Self::STRONG,
            'V' => Self::NotU,
            'T' => Self::THYMINE,
            'W' => Self::WEAK,
            'Y' => Self::PYRIMIDINE,
            'H' => Self::NotG,
            'K' => Self::KETO,
            'D' => Self::NotC,
            'B' => Self::NotA,
            'N' => Self::ANY,
            _ => Self::ZERO,
        }
    }
}

impl From<Codon> for char {
    fn from(c: Codon) -> Self {
        return CODONS[c as usize];
    }
}

impl From<u8> for Codon {
    fn from(u: u8) -> Self {
        if u < 16 {
            return Self::from(CODONS[u as usize]);
        } else {
            return Self::default();
        }
    }
}

impl PartialEq for Codon {
    fn eq(&self, other: &Self) -> bool {
        return *self as u8 & *other as u8 != 0;
    }
}

impl Into<u8> for Codon {
    fn into(self) -> u8 {
        match self {
            Codon::ZERO => 0x0,
            Codon::ADENINE => 0x1,
            Codon::CYTOSINE => 0x2,
            Codon::AMINO => 0x3,
            Codon::GUANINE => 0x4,
            Codon::PURINE => 0x5,
            Codon::STRONG => 0x6,
            Codon::NotU => 0x7,
            Codon::THYMINE => 0x8,
            Codon::WEAK => 0x9,
            Codon::PYRIMIDINE => 0xA,
            Codon::NotG => 0xB,
            Codon::KETO => 0xC,
            Codon::NotC => 0xD,
            Codon::NotA => 0xE,
            Codon::ANY => 0xF
        }
    }
}