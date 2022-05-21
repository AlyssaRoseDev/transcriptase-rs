use std::str::FromStr;

use either::Either;
use nom::{bytes::complete::is_a, Parser};

use crate::err::{TXError, TXResult};

use self::parsers::strand;
mod parsers;

#[derive(Debug, Clone)]
pub struct GFF {
    meta: Vec<Metadata>,
    entries: Vec<Entry>,
}

impl GFF {
    pub fn parse(src: &str) -> TXResult<Self> {
        let mut meta = Vec::new();
        let mut entries = Vec::new();
        for line in src.lines() {
            if line.starts_with('#') {
                if let Some(metadata) = Metadata::parse(line)? {
                    meta.push(metadata);
                }
            } else {
                entries.push(Entry::parse(line)?);
            }
        }

        Ok(Self { meta, entries })
    }
}

#[derive(Debug, Clone)]
pub enum Metadata {
    Pragma(String),
    Other(String),
}

impl Metadata {
    pub(crate) fn parse(src: &str) -> TXResult<Option<Self>> {
        let (tag, meta) = is_a("#").parse(src)?;
        Ok(match tag {
            "##" => Some(Self::Pragma(String::from(meta))),
            "#" => Some(Self::Other(String::from(meta))),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub(crate) seq_id: Box<str>,
    pub(crate) source: Box<str>,
    pub(crate) feature_type: Box<str>,
    pub(crate) range: (usize, usize),
    pub(crate) score: Option<f64>,
    pub(crate) strand: char,
    pub(crate) phase: Option<u8>,
    pub(crate) attrs: (Option<Id>, Vec<Attribute>),
}

impl Entry {
    // GFF Entry line:
    // {seq_id} {source} {type} {start} {end} {score?} {strand} {phase?} {attributes[]}
    pub(crate) fn parse(src: &str) -> TXResult<Self> {
        let (_, raw) = parsers::entry(src)?;
        let (seq, source, feature_type, range_start, range_end, score, strand, phase, attributes) =
            raw;
        let mut id = None;
        let mut attrs = Vec::new();
        for &attr in &attributes {
            match Attribute::parse(attr)? {
                Either::Left(attribute) => attrs.push(attribute),
                Either::Right(id_attr) => {
                    if id.is_none() {
                        id = Some(id_attr);
                    } else {
                        return Err(TXError::DuplicateGFFEntryID());
                    }
                }
            }
        }
        Ok(Self {
            seq_id: String::from(seq).into_boxed_str(),
            source: String::from(source).into_boxed_str(),
            feature_type: String::from(feature_type).into_boxed_str(),
            range: (range_start, range_end),
            score,
            strand,
            phase,
            attrs: (id, attrs),
        })
    }
}

impl FromStr for Entry {
    type Err = TXError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Entry::parse(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Box<str>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    Name(Box<str>),
    Alias(Box<str>),
    Parent(Vec<Id>),
    Target {
        target_id: Id,
        start: usize,
        end: usize,
        strand: Option<char>,
    },
    Gap(char),
    DerivesFrom(Id),
    Note(Box<str>),
    DbxRef(Box<str>),
    OntologyTerm(Box<str>),
    IsCircular(bool),
    Other(Box<str>),
}

impl Attribute {
    pub(crate) fn parse(src: &str) -> TXResult<Either<Self, Id>> {
        let (tag, value) = src.split_once('=').ok_or_else(|| {
            TXError::NomParsing(format!("Invalid attribute, expected tag=value, got {src}"))
        })?;
        Ok(if tag == "ID" {
            Either::Right(Id(value.to_string().into_boxed_str()))
        } else {
            Either::Left(Self::parse_kind(tag, value)?)
        })
    }

    pub(crate) fn parse_kind(src: &str, value: &str) -> TXResult<Self> {
        Ok(match src {
            "Name" => Self::Name(value.to_string().into_boxed_str()),
            "Alias" => Self::Alias(value.to_string().into_boxed_str()),
            "Parent" => Self::Parent(value.split(',').map(|s| Id(s.to_string().into_boxed_str())).collect()),
            "Target" => {
                let mut parts = value.split(' ');
                let target_id = Id(parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Target_Id ({src}={value})")))?.to_string().into_boxed_str());
                let start = parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Start ({src}={value})")))?.parse()?;
                let end = parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing End ({src}={value})")))?.parse()?;
                let strand = if let Some(st) = parts.next() {
                    Some(strand(st).map(|(_, strand)| strand)?)
                } else {
                    None
                };
                Self::Target { target_id, start, end, strand }
            },
            "Gap" => todo!(),
            "Derives_from" => todo!(),
            "Note" => todo!(),
            "Dbxref" => todo!(),
            "Ontology_term" => todo!(),
            "Is_circular" => todo!(),
            tag if tag.chars().next().ok_or_else(|| TXError::InvalidAttribute(String::from("Got empty Attribute Tag")))?.is_ascii_uppercase() => return Err(TXError::InvalidAttribute(format!("Attribute tags that start with an uppercase letter must match one of the official attributes, got {tag}"))),
            _ => Self::Other(String::from(src).into_boxed_str()),
        })
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use nom::{
        bytes::complete::tag,
        sequence::{terminated, tuple},
        Parser,
    };
    use nom_supreme::ParserExt;

    use crate::err::TXResult;

    const TEST_ENTRY: &str = "NC_045512.2	RefSeq	region	1	29903	.	+	.	ID=NC_045512.2:1..29903;Dbxref=taxon:2697049;collection-date=Dec-2019;country=China;gb-acronym=SARS-CoV-2;gbkey=Src;genome=genomic;isolate=Wuhan-Hu-1;mol_type=genomic RNA;nat-host=Homo sapiens;old-name=Wuhan seafood market pneumonia virus";
    const TEST_ENTRY_TWO: &str =
        r#"NC_045512.2	RefSeq	five_prime_UTR	1	265	.	+	.	ID=id-NC_045512.2:1..265;gbkey=5'UTR"#;

    #[test]
    fn seq_id_test() {
        let res = super::parsers::seq_id(TEST_ENTRY);
        if let Ok((rem, seq)) = res {
            println!("Sequence ID:");
            println!("{seq}");
            println!("Remainder:");
            println!("{rem}");
        }
    }

    #[test]
    fn tuple_test() {
        let res = tuple((
            terminated(super::parsers::seq_id, tag("\t")),
            super::parsers::source,
        ))
        .parse(TEST_ENTRY);
        if let Ok((rem, (seq, source))) = res {
            println!("Sequence ID:");
            println!("{seq}");
            println!("Source:");
            println!("{source}");
            println!("Remainder:");
            println!("{rem}");
        }
    }
    #[test]
    fn optionals() {
        let res = tuple((
            terminated(super::parsers::seq_id, tag("\t")),
            terminated(super::parsers::source, tag("\t")),
            terminated(super::parsers::feature_type, tag("\t")),
            terminated(super::parsers::range_bound, tag("\t")),
            terminated(super::parsers::range_bound, tag("\t")),
            terminated(super::parsers::score, tag("\t")),
            terminated(super::parsers::strand, tag("\t")),
            terminated(super::parsers::phase, tag("\t")),
            super::parsers::attributes,
        ))
        .all_consuming()
        .parse(TEST_ENTRY_TWO);
        let (_, (seq, source, feature_type, range_start, range_end, score, strand, phase, attrs)) =
            res.unwrap();
        println!("Sequence ID:");
        println!("{seq}");
        println!("Source:");
        println!("{source}");
        println!("Feature Type:");
        println!("{feature_type}");
        println!("Range:");
        println!("{range_start} -> {range_end}");
        println!("Score:");
        println!("{score:?}");
        println!("Strand:");
        println!("{strand}");
        println!("Phase:");
        println!("{phase:?}");
        println!("Attributes:");
        println!("{attrs:?}");
    }

    #[test]
    fn full() -> TXResult<()> {
        let mut file = std::fs::File::open(
            r#"E:\Projects\sars-cov-2\transcriptase\GCF_009858895.2_ASM985889v3_genomic.gff"#,
        )?;
        let mut src = String::with_capacity(file.metadata()?.len().try_into()?);
        file.read_to_string(&mut src)?;
        super::GFF::parse(&src)?;
        Ok(())
    }
}
