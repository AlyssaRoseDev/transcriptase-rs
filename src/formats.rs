use std::fs::File;

use crate::{genomics::genome::Genome, error::{TxaseErr, TxaseResult}};

pub trait Format{
    type OutFormat;
    fn serialize(self) -> Self::OutFormat;
    fn deserialize(de: Self::OutFormat) -> Self;
}

pub struct Fasta{
    file: File,
    formatted: Genome
}