use std::fmt::Display;
use std::{result::Result, str::FromStr};

/*
These hex representations allow for the representation of IUPAC extended Codons to match to the parallel
bitwise operations on the Nucleobases, e.g. Amino(M) to be represented as the bitwise OR of Adenine and
Cytosine. The special case is the NotX representations, which are the bitwise not of the nucleobase
followed by the bitwise AND of 0xF to ensure the representation conforms to 4 bits per Codon.
*/

///CODONS stores the ASCII encoding of the representative characters for each supported nucleobase
pub(crate) const DNA_CODONS: [char; 16] = [
    '0', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'T', 'W', 'Y', 'H', 'K', 'D', 'B', 'N',
];

/// The 16 degenerate base symbols that can occur in DNA as defined by the ["Nomenclature for incompletely specified bases in nucleic acid sequences: recommendations 1984"](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC322779)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DNA {
    /// A gap in the sequence
    Gap = 0x0,
    /// An Adenine nucleotide
    Adenine = 0x1,
    /// A Cytosine nucleotide
    Cytosine = 0x2,
    /// Either Adenine or Cytosine
    Amino = 0x3,
    /// A Guanine nucleotide
    Guanine = 0x4,
    /// Either Adenine or Guanine
    Purine = 0x5,
    /// Either Cytosine or Guanine
    Strong = 0x6,
    /// Any one of Adenine, Cytosine, or Guanine
    NotT = 0x7,
    /// A Thymine nucleotide
    Thymine = 0x8,
    /// Either Adenine or Thymine
    Weak = 0x9,
    /// Either Cytosine or Thymine
    Pyrimidine = 0xA,
    /// Any one of Adenine, Cytosine, or Thymine
    NotG = 0xB,
    /// Either Guanine or Thymine
    Ketone = 0xC,
    /// Any one of Adenine, Guanine, or Thymine
    NotC = 0xD,
    /// Any one of Cytosine, Guanine, or Thymine
    NotA = 0xE,
    /// Any DNA nucleotide
    #[default]
    Any = 0xF,
}

impl Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", DNA_CODONS[*self as usize])
    }
}

impl FromStr for DNA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(DNA::Gap),
            "A" => Ok(DNA::Adenine),
            "C" => Ok(DNA::Cytosine),
            "M" => Ok(DNA::Amino),
            "G" => Ok(DNA::Guanine),
            "R" => Ok(DNA::Purine),
            "S" => Ok(DNA::Strong),
            "V" => Ok(DNA::NotT),
            "T" => Ok(DNA::Thymine),
            "W" => Ok(DNA::Weak),
            "Y" => Ok(DNA::Pyrimidine),
            "H" => Ok(DNA::NotG),
            "K" => Ok(DNA::Ketone),
            "D" => Ok(DNA::NotC),
            "B" => Ok(DNA::NotA),
            "N" => Ok(DNA::Any),
            _ => Err(format!("Invalid Nucleotide: {s}")),
        }
    }
}

impl TryFrom<char> for DNA {
    type Error = String;

    fn try_from(value: char) -> Result<Self, String> {
        Ok(match value {
            '0' => Self::Gap,
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'M' => Self::Amino,
            'G' => Self::Guanine,
            'R' => Self::Purine,
            'S' => Self::Strong,
            'V' => Self::NotT,
            'T' => Self::Thymine,
            'W' => Self::Weak,
            'Y' => Self::Pyrimidine,
            'H' => Self::NotG,
            'K' => Self::Ketone,
            'D' => Self::NotC,
            'B' => Self::NotA,
            'N' => Self::Any,
            _ => return Err(format!("Expected one of ['0', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'T', 'W', 'Y', 'H', 'K', 'D', 'B', 'N'], got {value}")),
        })
    }
}

impl TryFrom<u8> for DNA {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, String> {
        Ok(match value {
            0x0 => Self::Gap,
            0x1 => Self::Adenine,
            0x2 => Self::Cytosine,
            0x3 => Self::Amino,
            0x4 => Self::Guanine,
            0x5 => Self::Purine,
            0x6 => Self::Strong,
            0x7 => Self::NotT,
            0x8 => Self::Thymine,
            0x9 => Self::Weak,
            0xA => Self::Pyrimidine,
            0xB => Self::NotG,
            0xC => Self::Ketone,
            0xD => Self::NotC,
            0xE => Self::NotA,
            0xF => Self::Any,
            _ => return Err(format!("Invalid Nucleotide: {value}")),
        })
    }
}

impl From<DNA> for char {
    fn from(dna: DNA) -> Self {
        DNA_CODONS[dna as usize]
    }
}

impl From<&DNA> for char {
    fn from(dna: &DNA) -> Self {
        DNA_CODONS[*dna as usize]
    }
}

impl From<&mut DNA> for char {
    fn from(dna: &mut DNA) -> Self {
        DNA_CODONS[*dna as usize]
    }
}

pub(crate) const RNA_CODONS: [char; 16] = [
    '0', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'U', 'W', 'Y', 'H', 'K', 'D', 'B', 'N',
];

/// The 16 degenerate base symbols that can occur in RNA as defined by the ["Nomenclature for incompletely specified bases in nucleic acid sequences: recommendations 1984"](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC322779)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RNA {
    /// A gap in the sequence
    Zero = 0x0,
    /// An Adenine nucleotide
    Adenine = 0x1,
    /// A Cytosine nucleotide
    Cytosine = 0x2,
    /// Either Adenine or Cytosine
    Amino = 0x3,
    /// A Guanine nucleotide
    Guanine = 0x4,
    /// Either Adenine or Guanine
    Purine = 0x5,
    /// Either Cytosine or Guanine
    Strong = 0x6,
    /// Any one of Adenine, Cytosine, or Guanine
    NotU = 0x7,
    /// A Uracil nucleotide
    Uracil = 0x8,
    /// Either Adenine or Uracil
    Weak = 0x9,
    /// Either Cytosine or Uracil
    Pyrimidine = 0xA,
    /// Any one of Adenine, Cytosine, or Uracil
    NotG = 0xB,
    /// Either Guanine or Uracil
    Ketone = 0xC,
    /// Any one of Adenine, Guanine, or Uracil
    NotC = 0xD,
    /// Any one of Cytosine, Guanine, or Uracil
    NotA = 0xE,
    /// Any DNA nucleotide
    #[default]
    Any = 0xF,
}

impl Display for RNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", RNA_CODONS[*self as usize])
    }
}

impl FromStr for RNA {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(RNA::Zero),
            "A" => Ok(RNA::Adenine),
            "C" => Ok(RNA::Cytosine),
            "M" => Ok(RNA::Amino),
            "G" => Ok(RNA::Guanine),
            "R" => Ok(RNA::Purine),
            "S" => Ok(RNA::Strong),
            "V" => Ok(RNA::NotU),
            "U" => Ok(RNA::Uracil),
            "W" => Ok(RNA::Weak),
            "Y" => Ok(RNA::Pyrimidine),
            "H" => Ok(RNA::NotG),
            "K" => Ok(RNA::Ketone),
            "D" => Ok(RNA::NotC),
            "B" => Ok(RNA::NotA),
            "N" => Ok(RNA::Any),
            _ => Err(format!("Invalid Nucleotide: {s}")),
        }
    }
}

impl TryFrom<char> for RNA {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Zero,
            'A' => Self::Adenine,
            'C' => Self::Cytosine,
            'M' => Self::Amino,
            'G' => Self::Guanine,
            'R' => Self::Purine,
            'S' => Self::Strong,
            'V' => Self::NotU,
            'U' => Self::Uracil,
            'W' => Self::Weak,
            'Y' => Self::Pyrimidine,
            'H' => Self::NotG,
            'K' => Self::Ketone,
            'D' => Self::NotC,
            'B' => Self::NotA,
            'N' => Self::Any,
            _ => return Err(format!("Expected one of ['0', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'U', 'W', 'Y', 'H', 'K', 'D', 'B', 'N'], got {value}")),
        })
    }
}

impl TryFrom<u8> for RNA {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, String> {
        Ok(match value {
            0x0 => Self::Zero,
            0x1 => Self::Adenine,
            0x2 => Self::Cytosine,
            0x3 => Self::Amino,
            0x4 => Self::Guanine,
            0x5 => Self::Purine,
            0x6 => Self::Strong,
            0x7 => Self::NotU,
            0x8 => Self::Uracil,
            0x9 => Self::Weak,
            0xA => Self::Pyrimidine,
            0xB => Self::NotG,
            0xC => Self::Ketone,
            0xD => Self::NotC,
            0xE => Self::NotA,
            0xF => Self::Any,
            _ => return Err(format!("Invalid Nucleotide: {value}")),
        })
    }
}

impl From<RNA> for char {
    fn from(rna: RNA) -> Self {
        RNA_CODONS[rna as usize] as char
    }
}

impl From<&RNA> for char {
    fn from(rna: &RNA) -> Self {
        RNA_CODONS[*rna as usize] as char
    }
}

impl From<&mut RNA> for char {
    fn from(rna: &mut RNA) -> Self {
        RNA_CODONS[*rna as usize] as char
    }
}
