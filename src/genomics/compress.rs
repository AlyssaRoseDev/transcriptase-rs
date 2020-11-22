use super::codon::Codon;

pub const MASK: u8 = 0xF;
#[derive(Debug, Clone)]
pub(crate) struct CompressedCodon(u8);

impl CompressedCodon {
    pub fn compress(c1: Codon, c2: Codon) -> CompressedCodon {
        let (low, high): (u8, u8) = (c1.into(), c2.into());
        return CompressedCodon((high << 4) | low);
    }
    pub fn split(self) -> (Codon, Codon) {
        let low = self.clone().0 & MASK;
        let high = (self.0 >> 4) & MASK;
        return (Codon::from(low), Codon::from(high))
    }
}

#[derive(Debug, Clone)]
#[repr(packed(1))]
pub(crate) struct PackedCodon(u8);