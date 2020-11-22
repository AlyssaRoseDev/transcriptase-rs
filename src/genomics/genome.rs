use super::codon::Codon;
use crate::error::TxaseResult;
use std::{fmt::Display, ops::Index, fs::{File, OpenOptions}, io::{BufRead, BufReader}, path::Path};

#[derive(Debug)]
pub struct Genome {
    nucleotides: Vec<Codon>,
}

impl Genome {
    pub fn open_and_parse(
        path: impl AsRef<Path>,
        openops: Option<OpenOptions>,
    ) -> TxaseResult<Genome> {
        let f: File = if openops.is_some() {
            openops.unwrap().open(path)?
        } else {
            File::open(path)?
        };
        let size = f.metadata().unwrap().len() as usize;
        let b_read = BufReader::new(f);
        let gen = Genome::parse_seq(BufRead::lines(b_read).map(|x| x.unwrap()), size);
        return Ok(gen);
    }

    pub fn parsemultiseq(path: impl AsRef<Path>, openops: Option<OpenOptions>) -> Vec<Genome> {
        let gv: Vec<Genome> = Vec::new();
        return gv;
    }

    pub fn parse_seq(lines: impl Iterator<Item = String>, len: usize) -> Genome {
        let mut gen = Genome {
            nucleotides: Vec::with_capacity(len),
        };
        for line in lines {
            if line.starts_with(">") || line.starts_with(";") {
                continue;
            }
            for c in line.chars() {
                gen.add(c)
            }
        }
        return gen;
    }

    pub fn add(&mut self, i: impl Into<Codon>) {
        self.nucleotides.push(i.into())
    }
}

impl Display for Genome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut cnt: u8 = 0;
        for n in self.nucleotides.iter() {
            s.push((*n).into());
            cnt += 1;
            if cnt == 60 {
                cnt = 0;
                s.push('\n')
            }
        }
        return write!(f, "{}", s);
    }
}