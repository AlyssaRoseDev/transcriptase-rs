use super::AminoAcid::{self, *};
use phf::{phf_map, Map};

pub(crate) static RNA_TRANSLATION_TABLE: Map<&str, AminoAcid> = phf_map! {
    "UUU" => Phenylalanine,
    "UUC" => Phenylalanine,
    "UUA" => Leucine,
    "UUG" => Leucine,
    "CUU" => Leucine,
    "CUC" => Leucine,
    "CUA" => Leucine,
    "CUG" => Leucine,
    "AUU" => Isoleucine,
    "AUC" => Isoleucine,
    "AUA" => Isoleucine,
    "AUG" => Methionine,
    "GUU" => Valine,
    "GUC" => Valine,
    "GUA" => Valine,
    "GUG" => Valine,
    "UCU" => Serine,
    "UCC" => Serine,
    "UCA" => Serine,
    "UCG" => Serine,
    "CCU" => Proline,
    "CCC" => Proline,
    "CCA" => Proline,
    "CCG" => Proline,
    "ACU" => Threonine,
    "ACC" => Threonine,
    "ACA" => Threonine,
    "ACG" => Threonine,
    "GCU" => Alanine,
    "GCC" => Alanine,
    "GCA" => Alanine,
    "GCG" => Alanine,
    "UAU" => Tyrosine,
    "UAC" => Tyrosine,
    "UAA" => Stop,
    "UAG" => Stop,
    "CAU" => Histidine,
    "CAC" => Histidine,
    "CAA" => Glutamine,
    "CAG" => Glutamine,
    "AAU" => Asparagine,
    "AAC" => Asparagine,
    "AAA" => Lysine,
    "AAG" => Lysine,
    "GAU" => Aspartate,
    "GAC" => Aspartate,
    "GAA" => Glutamate,
    "GAG" => Glutamate,
    "UGU" => Cysteine,
    "UGC" => Cysteine,
    "UGA" => Stop,
    "UGG" => Tryptonphan,
    "CGU" => Arginine,
    "CGC" => Arginine,
    "CGA" => Arginine,
    "CGG" => Arginine,
    "AGU" => Serine,
    "AGC" => Serine,
    "AGA" => Arginine,
    "AGG" => Arginine,
    "GGU" => Glycine,
    "GGC" => Glycine,
    "GGA" => Glycine,
    "GGG" => Glycine,
};

pub(crate) static DNA_TRANSLATION_TABLE: Map<&str, AminoAcid> = phf_map! {
    "TTT" => Phenylalanine,
    "TTC" => Phenylalanine,
    "TTA" => Leucine,
    "TTG" => Leucine,
    "CTT" => Leucine,
    "CTC" => Leucine,
    "CTA" => Leucine,
    "CTG" => Leucine,
    "ATT" => Isoleucine,
    "ATC" => Isoleucine,
    "ATA" => Isoleucine,
    "ATG" => Methionine,
    "GTT" => Valine,
    "GTC" => Valine,
    "GTA" => Valine,
    "GTG" => Valine,
    "TCT" => Serine,
    "TCC" => Serine,
    "TCA" => Serine,
    "TCG" => Serine,
    "CCT" => Proline,
    "CCC" => Proline,
    "CCA" => Proline,
    "CCG" => Proline,
    "ACT" => Threonine,
    "ACC" => Threonine,
    "ACA" => Threonine,
    "ACG" => Threonine,
    "GCT" => Alanine,
    "GCC" => Alanine,
    "GCA" => Alanine,
    "GCG" => Alanine,
    "TAT" => Tyrosine,
    "TAC" => Tyrosine,
    "TAA" => Stop,
    "TAG" => Stop,
    "CAT" => Histidine,
    "CAC" => Histidine,
    "CAA" => Glutamine,
    "CAG" => Glutamine,
    "AAT" => Asparagine,
    "AAC" => Asparagine,
    "AAA" => Lysine,
    "AAG" => Lysine,
    "GAT" => Aspartate,
    "GAC" => Aspartate,
    "GAA" => Glutamate,
    "GAG" => Glutamate,
    "TGT" => Cysteine,
    "TGC" => Cysteine,
    "TGA" => Stop,
    "TGG" => Tryptonphan,
    "CGT" => Arginine,
    "CGC" => Arginine,
    "CGA" => Arginine,
    "CGG" => Arginine,
    "AGT" => Serine,
    "AGC" => Serine,
    "AGA" => Arginine,
    "AGG" => Arginine,
    "GGT" => Glycine,
    "GGC" => Glycine,
    "GGA" => Glycine,
    "GGG" => Glycine,
};
