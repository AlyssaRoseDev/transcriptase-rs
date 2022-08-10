use crate::{
    err::{TXaseError, TXaseResult},
    gff::{parsers::strand, Strand},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct AttributeSet {
    pub id: Option<Id>,
    pub name: Option<Box<str>>,
    pub alias: Option<Box<str>>,
    pub parent: Option<Vec<Id>>,
    pub target: Option<TargetAttr>,
    pub gap: Option<Vec<(GapKind, usize)>>,
    pub derives_from: Option<Id>,
    pub note: Option<Box<str>>,
    pub dbx_ref: Option<Box<str>>,
    pub ontology_term: Option<Box<str>>,
    pub is_circular: Option<()>,
    pub other: Option<Vec<(Box<str>, Box<str>)>>,
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
        "Name" => attrs.name = Some(value.into()),
        "Alias" => attrs.alias = Some(value.into()),
        "Parent" => attrs.parent = Some(value.split(',').map(Id::from).collect()),
        "Target" => {
            let mut parts = value.split(' ');
            let target_id = parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Target_Id ({src}={value})")))?.into();
            let start = parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing Start ({src}={value})")))?.parse()?;
            let end = parts.next().ok_or_else(|| TXaseError::InvalidAttribute(format!("Unexpected end of Target Attribute, missing End ({src}={value})")))?.parse()?;
            let strand = parts.next().map(|st| strand(st).map(|(_, strand)| strand)).transpose()?.flatten().map(Strand::parse).transpose()?;
            attrs.target = Some(TargetAttr { target_id, start, end, strand })
        },
        "Gap" => attrs.gap = Some(value.split(' ').map(|gap| {
            let (kind, len) = gap.split_at(0);
            Ok((GapKind::parse(kind)?, len.parse::<usize>()?))
        }).collect::<TXaseResult<Vec<_>>>()?),
        "Derives_from" => attrs.derives_from = Some(value.into()),
        "Note" => attrs.note = Some(value.into()),
        "Dbxref" => attrs.dbx_ref = Some(value.into()),
        "Ontology_term" => attrs.ontology_term = Some(value.into()),
        "Is_circular" => match value {
            "true" => attrs.is_circular = Some(()),
            "false" => continue,
            val => return Err(TXaseError::InvalidAttribute(format!("Invalid Is_circular attribute expected one of ['true', 'false'], got: {val}")))
        },
        tag if tag.chars().next().ok_or_else(|| TXaseError::InvalidAttribute(String::from("Got empty Attribute Tag")))?.is_ascii_uppercase() => return Err(TXaseError::InvalidAttribute(format!("Attribute tags that start with an uppercase letter must match one of the official attributes, got {tag}"))),
        tag => attrs.other.get_or_insert(Vec::new()).push((tag.into(), value.into())),
    }
        }
        Ok(attrs)
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
