use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
    character::complete::{char, one_of},
    combinator::{eof, map, value},
    error::VerboseError,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    Parser,
};
use std::{borrow::Borrow, fmt, str::FromStr};
use tracing::trace;

use nom::{
    bytes::complete::is_not,
    character::complete::digit1,
    combinator::map_res,
    sequence::{terminated, tuple},
};
use nom_supreme::{
    final_parser::final_parser, multi::parse_separated_terminated_res, tag::complete::tag,
};

use crate::{
    gff::{
        parsers::{strand, ParseError},
        Strand,
    },
    NomResult,
};

use super::{GffError, UnescapedString};

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

impl AttributeSet {
    #[tracing::instrument]
    pub(crate) fn parse(src: &str) -> Result<Self, GffError> {
        let attrs =
            final_parser::<_, _, VerboseError<&str>, ParseError>(parse_separated_terminated_res(
                Self::get_key_val,
                tag(";"),
                eof,
                AttributeSet::default,
                |mut attrs, (key, val)| {
                    match key {
                        "ID" => attrs.id = Some(Id::new(val)),
                        "Name" => attrs.name = Some(UnescapedString::new(val)?),
                        "Alias" => attrs.alias = Some(UnescapedString::new(val)?),
                        "Parent" => {
                            attrs.parent =
                                Some(final_parser::<_, _, VerboseError<&str>, ParseError>(
                                    separated_list1(char(','), map(is_not(","), Id::new)),
                                )(val)?)
                        }
                        "Target" => {
                            attrs.target =
                                Some(final_parser::<_, _, VerboseError<&str>, ParseError>(
                                    TargetAttr::parse,
                                )(val)?)
                        }
                        "Gap" => {
                            attrs.gap = Some(final_parser::<_, _, VerboseError<&str>, ParseError>(
                                separated_list1(
                                    char(' '),
                                    pair(
                                        map(one_of("MIDFR"), GapKind::parse_unwrap),
                                        map_res(digit1::<&str, _>, usize::from_str),
                                    ),
                                ),
                            )(val)?)
                        }
                        "Derives_from" => attrs.derives_from = Some(Id::new(val)),
                        "Note" => attrs.note = Some(UnescapedString::new(val)?),
                        "Dbxref" => attrs.dbx_ref = Some(UnescapedString::new(val)?),
                        "Ontology_term" => attrs.ontology_term = Some(UnescapedString::new(val)?),
                        "Is_circular" => {
                            attrs.is_circular =
                                final_parser::<_, _, VerboseError<&str>, ParseError>(map(
                                    alt((value(true, tag("true")), value(false, tag("false")))),
                                    Some,
                                ))(val)?;
                        }
                        tag if tag
                            .chars()
                            .next()
                            .ok_or(GffError::MalformedLine)?
                            .is_ascii_uppercase() =>
                        {
                            return Err(GffError::ReservedAttribute)
                        }
                        tag => attrs
                            .other
                            .get_or_insert(Vec::new())
                            .push((tag.into(), UnescapedString::new(val)?)),
                    };
                    Ok(attrs)
                },
            ))(src)?;
        tracing::debug!("{}", attrs);
        Ok(attrs)
    }

    #[tracing::instrument]
    fn get_key_val<'source, 'ret>(chunk: &'source str) -> NomResult<'ret, (&'ret str, &'ret str)>
    where
        'source: 'ret,
    {
        //todo: make this detect actual allowed and not allowed things :3
        let (ret, this) =
            separated_pair(take_until("="), char('='), take_while(|c| c != ';')).parse(chunk)?;
        trace!("Returning ({}, {}), Remaining: \"{ret}\"", this.0, this.1);
        Ok((ret, this))
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

impl TargetAttr {
    fn parse(src: &str) -> NomResult<'_, Self> {
        tuple((
            map(terminated(is_not(" "), tag(" ")), Id::new),
            map_res(terminated(digit1, tag(" ")), FromStr::from_str),
            map_res(terminated(digit1, tag(" ")), FromStr::from_str),
            map_res(strand, |s| s.map(Strand::parse).transpose()),
        ))
        .map(|(target_id, start, end, strand)| Self {
            target_id,
            start,
            end,
            strand,
        })
        .parse(src)
    }
}

impl fmt::Display for TargetAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
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
    pub fn parse(src: char) -> Result<Self, GffError> {
        Ok(match src {
            'M' => Self::Match,
            'I' => Self::Insert,
            'D' => Self::Delete,
            'F' => Self::FwdFrameShift,
            'R' => Self::RevFrameShift,
            _ => return Err(GffError::InvalidGapKind),
        })
    }

    fn parse_unwrap(src: char) -> Self {
        Self::parse(src).unwrap()
    }
}
