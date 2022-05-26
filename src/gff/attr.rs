use crate::{
    err::{TXError, TXResult},
    gff::{parsers::strand, Strand},
};
use either::Either;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    Name(Box<str>),
    Alias(Box<str>),
    Parent(Vec<Id>),
    Target {
        target_id: Id,
        start: usize,
        end: usize,
        strand: Option<Strand>,
    },
    Gap(Vec<(GapKind, usize)>),
    DerivesFrom(Id),
    Note(Box<str>),
    DbxRef(Box<str>),
    OntologyTerm(Box<str>),
    IsCircular(bool),
    Other(Box<str>, Box<str>),
}

impl Attribute {
    pub(crate) fn parse(src: &str) -> TXResult<Either<Self, Id>> {
        let (tag, value) = src.split_once('=').ok_or_else(|| {
            TXError::InternalParseFailure(format!(
                "Invalid attribute, expected tag=value, got {src}"
            ))
        })?;
        Ok(if tag == "ID" {
            Either::Right(value.into())
        } else {
            Either::Left(Self::parse_kind(tag, value)?)
        })
    }

    pub(crate) fn parse_kind(src: &str, value: &str) -> TXResult<Self> {
        Ok(match src {
        "Name" => Self::Name(src.into()),
        "Alias" => Self::Alias(src.into()),
        "Parent" => Self::Parent(value.split(',').map(Id::from).collect()),
        "Target" => {
            let mut parts = value.split(' ');
            let target_id = parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Target_Id ({src}={value})")))?.into();
            let start = parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Start ({src}={value})")))?.parse()?;
            let end = parts.next().ok_or_else(|| TXError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing End ({src}={value})")))?.parse()?;
            let strand = parts.next().map(|st| strand(st).map(|(_, strand)| strand)).transpose()?.flatten().map(Strand::parse).transpose()?;
            Self::Target { target_id, start, end, strand }
        },
        "Gap" => Self::Gap(src.split(' ').map(|gap| -> TXResult<(GapKind, usize)> {
            let (kind, len) = gap.split_at(0);
            Ok((GapKind::parse(kind)?, len.parse::<usize>()?))
        }).collect::<TXResult<Vec<(GapKind, usize)>>>()?),
        "Derives_from" => Self::DerivesFrom(value.into()),
        "Note" => Self::Note(value.into()),
        "Dbxref" => Self::DbxRef(value.into()),
        "Ontology_term" => Self::OntologyTerm(value.into()),
        "Is_circular" => match value {
            "true" => Self::IsCircular(true),
            "false" => Self::IsCircular(false),
            val => return Err(TXError::InvalidAttribute(format!("Invalid Is_circular attribute expected one of ['true', 'false'], got: {val}")))
        },
        tag if tag.chars().next().ok_or_else(|| TXError::InvalidAttribute(String::from("Got empty Attribute Tag")))?.is_ascii_uppercase() => return Err(TXError::InvalidAttribute(format!("Attribute tags that start with an uppercase letter must match one of the official attributes, got {tag}"))),
        tag => Self::Other(tag.into(), value.into()),
    })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Box<str>);

impl<T> From<T> for Id
where
    T: ToString,
{
    fn from(src: T) -> Self {
        Self(src.to_string().into_boxed_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GapKind {
    Match,
    Insert,
    Delete,
    FwdFrameShift,
    RevFrameShift,
}

impl GapKind {
    pub fn parse(src: &str) -> TXResult<Self> {
        Ok(match src {
            "M" => Self::Match,
            "I" => Self::Insert,
            "D" => Self::Delete,
            "F" => Self::FwdFrameShift,
            "R" => Self::RevFrameShift,
            _ => {
                return Err(TXError::InvalidAttribute(format!(
                    "Invalid Gap Kind, expected one of ['M', 'I', 'D', 'F', 'R'], got: {src}"
                )))
            }
        })
    }
}
