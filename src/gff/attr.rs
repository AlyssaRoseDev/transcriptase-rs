use std::{borrow::Borrow, fmt};

use crate::{
    err::{TXaseError, TXaseResult},
    gff::{
        parsers::{strand, ParseError},
        Strand,
    },
};

use super::UnescapedString;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSet {
    pub id: Option<Id>,
    pub name: Option<UnescapedString>,
    pub alias: Option<UnescapedString>,
    pub parent: Option<Vec<Id>>,
    pub target: Option<TargetAttr>,
    pub gap: Option<Vec<(GapKind, usize)>>,
    pub derives_from: Option<Id>,
    pub note: Option<UnescapedString>,
    pub dbx_ref: Option<UnescapedString>,
    pub ontology_term: Option<UnescapedString>,
    pub is_circular: Option<bool>,
    pub other: Option<Vec<(Box<str>, UnescapedString)>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetAttr {
    target_id: Id,
    start: usize,
    end: usize,
    strand: Option<Strand>,
}

impl fmt::Display for TargetAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl AttributeSet {
    #[tracing::instrument]
    pub(crate) fn parse(src: &str) -> TXaseResult<Self> {
        let mut attrs = AttributeSet::default();
        let attr_iter = src.split(';').flat_map(|attr| {
            attr.split_once('=').ok_or_else(|| {
                TXaseError::InvalidAttribute(format!(
                    "Invalid attribute, expected tag=value, got {src}"
                ))
            })
        });
        for (tag, value) in attr_iter {
            match tag {
                "ID" => attrs.id = Some(Id::new(value)),
                "Name" => attrs.name = Some(UnescapedString::new(value)?),
                "Alias" => attrs.alias = Some(UnescapedString::new(value)?),
                "Parent" => attrs.parent = Some(value.split(',').map(Id::new).collect()),
                "Target" => {
                    let mut parts = value.split(' ');
                    let target_id = Id::new(parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Target_Id ({src}={value})")))?);
                    let start = parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Start ({src}={value})")))?.parse()?;
                    let end = parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing End ({src}={value})")))?.parse()?;
                    let strand = parts.next().map(|st| strand(st).map(|(_, strand)| strand)).transpose().map_err(|e| e.to_string())?.flatten().map(Strand::parse).transpose()?;
                    attrs.target = Some(TargetAttr { target_id, start, end, strand })
                },
                "Gap" => attrs.gap = Some(value.split(' ').map(|gap| {
                    let (kind, len) = gap.split_at(0);
                    Ok((GapKind::parse(kind)?, len.parse::<usize>()?))
                }).collect::<TXaseResult<Vec<_>>>()?),
                "Derives_from" => attrs.derives_from = Some(Id::new(value)),
                "Note" => attrs.note = Some(UnescapedString::new(value)?),
                "Dbxref" => attrs.dbx_ref = Some(UnescapedString::new(value)?),
                "Ontology_term" => attrs.ontology_term = Some(UnescapedString::new(value)?),
                "Is_circular" => match value {
                    "true" => attrs.is_circular = Some(true),
                    "false" => attrs.is_circular = Some(false),
                    val => return Err(TXaseError::InvalidAttribute(format!("Invalid Is_circular attribute expected one of ['true', 'false'], got: {val}")))
                },
                tag if tag.chars().next().ok_or_else(|| TXaseError::InvalidAttribute(String::from("Got empty Attribute Tag")))?.is_ascii_uppercase() => return Err(TXaseError::InvalidAttribute(format!("Attribute tags that start with an uppercase letter must match one of the official attributes, got {tag}"))),
                tag => attrs.other.get_or_insert(Vec::new()).push((tag.into(), UnescapedString::new(value)?)),
            }
        }
        tracing::debug!("{}", attrs);
        Ok(attrs)
    }
}

macro_rules! opt_field {
    ($fmt:expr, $self:ident, $id:ident) => {
        if let Some(ref $id) = $self.$id {
            writeln!($fmt, "\t{}: {}", stringify!($id), $id)?;
        };
    };
}

impl fmt::Display for AttributeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AttributeSet {{")?;
        opt_field!(f, self, id);
        opt_field!(f, self, name);
        opt_field!(f, self, alias);
        if let Some(ref parents) = self.parent {
            let parentstr = parents.join(";");
            if parents.len() == 1 {
                writeln!(f, "\tParent: {}", parentstr)?;
            } else {
                writeln!(f, "\tParents: {}", parentstr)?;
            }
        };
        opt_field!(f, self, target);
        if let Some(ref gaps) = self.gap {
            if gaps.len() == 1 {
                let (kind, len) = gaps.get(0).expect("Length is 1");
                writeln!(f, "\tGap: {}{}", kind, len)?;
            } else {
                let gapstr = gaps
                    .iter()
                    .map(|(kind, len)| format!("{kind}{len}"))
                    .reduce(|prev, new| format!("{prev} {new}"))
                    .expect("self.gap is only ever Some if it's len is at least one");
                writeln!(f, "\tGaps: {}", gapstr)?;
            }
        };
        opt_field!(f, self, derives_from);
        opt_field!(f, self, note);
        opt_field!(f, self, dbx_ref);
        opt_field!(f, self, ontology_term);
        if let Some(circular) = self.is_circular {
            writeln!(f, "is_circular: {circular}")?;
        }
        if let Some(ref others) = self.other {
            for (key, val) in others {
                writeln!(f, "\t{}: {}", key, val)?;
            }
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Box<str>);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Borrow<str> for Id {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Id {
    fn new<T: ToString>(src: T) -> Self {
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

impl fmt::Display for GapKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            GapKind::Match => 'M',
            GapKind::Insert => 'I',
            GapKind::Delete => 'D',
            GapKind::FwdFrameShift => 'F',
            GapKind::RevFrameShift => 'R',
        };
        write!(f, "{}", c)
    }
}

impl GapKind {
    #[tracing::instrument]
    pub fn parse(src: &str) -> TXaseResult<Self> {
        Ok(match src {
            "M" => Self::Match,
            "I" => Self::Insert,
            "D" => Self::Delete,
            "F" => Self::FwdFrameShift,
            "R" => Self::RevFrameShift,
            _ => {
                return Err(TXaseError::InvalidAttribute(format!(
                    "Invalid Gap Kind, expected one of ['M', 'I', 'D', 'F', 'R'], got: {src}"
                )))
            }
        })
    }
}
