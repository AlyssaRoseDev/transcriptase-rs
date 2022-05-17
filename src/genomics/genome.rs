use super::codon::DNACodon;
use crate::err::TXResult;
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    ops::Index,
    path::Path,
};

#[derive(Debug)]
pub struct Genome {
    desc: String,
    nucleotides: Vec<DNACodon>,
}

impl Genome {
    pub fn parse(p: impl AsRef<Path>, ops: Option<OpenOptions>) -> TXResult<Self> {
        //decouple file parsing and data storage
        let file = if let Some(o) = ops {
            o.open(p)?
        } else {
            File::open(p)?
        };
        let mut gen = Genome {
            desc: String::from(""),
            nucleotides: Vec::with_capacity(file.metadata()?.len().try_into()?),
        };
        let buffer = BufReader::new(file);
        for l in buffer.lines() {
            let checked = l?;
            if checked.starts_with('>') || checked.starts_with(';') {
                gen.desc = checked;
                continue;
            }
            for c in checked.chars() {
                //gen.add(c)
            }
        }
        Ok(gen)
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
