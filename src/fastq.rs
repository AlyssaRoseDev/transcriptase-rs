use crate::err::{TXaseError, TXaseResult};
use crate::fasta::Sequence;
use quality::Quality;
pub use quality::{Phred, Solexa};

pub mod quality;

#[derive(Debug)]
pub struct FastQ<S: Sequence, Q: Quality> {
    pub description: Option<Box<str>>,
    pub sequence: Vec<(S::Inner, Q)>,
}

impl<S, Q> FastQ<S, Q>
where
    S: Sequence,
    Q: Quality,
    TXaseError: From<<Q as TryFrom<char>>::Error>,
    TXaseError: From<<S::Inner as TryFrom<char>>::Error>,
{
    pub fn parse(src: &str) -> TXaseResult<Self> {
        let mut lines = src.lines();
        let early_eof = || {
            TXaseError::InternalParseFailure(String::from(
                "Unexpected end of input in FastQ parsing!",
            ))
        };
        let line1 = lines.next().ok_or_else(early_eof)?;
        let line2 = lines.next().ok_or_else(early_eof)?;
        let line3 = lines.next().ok_or_else(early_eof)?;
        let line4 = lines.next().ok_or_else(early_eof)?;
        let desc = parsers::desc_line(line1, "@")?;
        let sec_desc = parsers::desc_line(line3, "+")?;
        if sec_desc != desc && sec_desc.is_some() {
            return Err(TXaseError::InternalParseFailure(format!(
                "Line three contained a description that did not match the original description!\nExpected: {desc:?}, Got: {sec_desc:?}"
            )));
        }
        let description: Option<Box<str>> = desc.map(Into::into);
        let sequence: Vec<(S::Inner, Q)> = line2
            .chars()
            .map(S::Inner::try_from)
            .collect::<Result<Vec<S::Inner>, _>>()?
            .into_iter()
            .zip(
                line4
                    .chars()
                    .map(Q::try_from)
                    .collect::<Result<Vec<Q>, _>>()?
                    .into_iter(),
            )
            .collect();
        Ok(Self {
            description,
            sequence,
        })
    }
}

mod parsers {
    use nom::{bytes::complete::tag, Parser};

    use crate::err::TXaseResult;

    pub fn desc_line<'src>(
        src: &'src str,
        desc_tag: &'static str,
    ) -> TXaseResult<Option<&'src str>> {
        tag(desc_tag)
            .parse(src)
            .map(|(desc, _)| if desc.is_empty() { None } else { Some(desc) })
            .map_err(Into::into)
    }
}
