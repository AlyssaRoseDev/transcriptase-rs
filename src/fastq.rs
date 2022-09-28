use std::collections::HashMap;

use crate::err::{TXaseError, TXaseResult};
use crate::fasta::Sequence;
use quality::Quality;
pub use quality::{Phred, Solexa};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub mod quality;

pub type Descriptor = String;
pub type QualitySequence<T, Q> = Vec<(T, Q)>;

#[derive(Debug)]
pub struct FastQ<S: Sequence, Q: Quality> {
    pub sequences: HashMap<Descriptor, QualitySequence<S::Inner, Q>>,
}

#[cfg(not(feature = "rayon"))]
impl<S, Q> FastQ<S, Q>
where
    S: Sequence,
    Q: Quality,
    TXaseError: From<<Q as TryFrom<char>>::Error>,
    TXaseError: From<<S::Inner as TryFrom<char>>::Error>,
{
    #[tracing::instrument(skip_all)]
    pub fn parse(src: &str) -> TXaseResult<Self> {
        let mut lines = src.lines();
        let sequences = std::iter::from_fn(move || {
            Some([lines.next()?, lines.next()?, lines.next()?, lines.next()?])
        })
        .map(Self::parse_single)
        .collect::<Result<HashMap<Descriptor, QualitySequence<S::Inner, Q>>, TXaseError>>()?;
        Ok(Self { sequences })
    }

    fn parse_single(set: [&str; 4]) -> TXaseResult<(Descriptor, QualitySequence<S::Inner, Q>)> {
        let desc = parsers::desc_line(set[0], "@")?.ok_or_else(|| {
            TXaseError::InternalParseFailure(
                "First description line must not be empty!".to_string(),
            )
        })?;
        match parsers::desc_line(set[2], "+")? {
            Some(s) if s != desc => Err(TXaseError::InternalParseFailure(format!("Line three contained a description that did not match the original description!\nExpected: {desc}, Got: {s}"))),
            _ => Ok((
                desc.to_string(),
                set[1]
                .chars()
                .zip(set[3].chars())
                .map(|(s, q)| -> TXaseResult<_> {
                    Ok((S::Inner::try_from(s)?, Q::try_from(q)?))
                })
                .collect::<TXaseResult<_>>()?))
        }
    }
}

#[cfg(feature = "rayon")]
impl<S, Q> FastQ<S, Q>
where
    S: Sequence,
    Q: Quality + Send,
    S::Inner: Send,
    <S::Inner as TryFrom<char>>::Error: Send,
    <Q as TryFrom<char>>::Error: Send,
    TXaseError: From<<Q as TryFrom<char>>::Error>,
    TXaseError: From<<S::Inner as TryFrom<char>>::Error>,
{
    #[tracing::instrument(skip_all)]
    pub fn parse(src: &str) -> TXaseResult<Self> {
        let mut lines = src.lines();
        let sequences = std::iter::from_fn(|| {
            Some([lines.next()?, lines.next()?, lines.next()?, lines.next()?])
        })
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(Self::parse_single)
        .collect::<Result<HashMap<Descriptor, QualitySequence<S::Inner, Q>>, TXaseError>>()?;
        Ok(Self { sequences })
    }

    fn parse_single(set: [&str; 4]) -> TXaseResult<(Descriptor, QualitySequence<S::Inner, Q>)> {
        let desc = parsers::desc_line(set[0], "@")?.ok_or_else(|| {
            TXaseError::InternalParseFailure(
                "First description line must not be empty!".to_string(),
            )
        })?;
        match parsers::desc_line(set[2], "+")? {
                        Some(s) if s != desc => Err(TXaseError::InternalParseFailure(format!("Line three contained a description that did not match the original description!\nExpected: {desc:?}, Got: {s:?}"))),
                        _ => Ok((
                            desc.to_string(),
                            set[1]
                                .as_bytes()
                                .into_par_iter()
                                .zip(set[3].as_bytes().into_par_iter())
                                .map(|(s, q)| -> TXaseResult<_> {
                               Ok((
                                    S::Inner::try_from(char::from(*s))?,
                                    Q::try_from(char::from(*q))?,
                                ))
                                })
                                .collect::<TXaseResult<_>>()?,
                        )),
                    }
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
