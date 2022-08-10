use std::collections::HashMap;
use std::str::Lines;

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
        let sequences = LineSets::new(src)
            .map(
                |line_set| -> Result<(Descriptor, QualitySequence<S::Inner, Q>), TXaseError> {
                    let desc = parsers::desc_line(line_set[0], "@")?
                        .ok_or_else(|| {
                            TXaseError::InternalParseFailure(format!(
                                "First description line must not be empty!"
                            ))
                        })
                        .and_then(
                            |first| if let Some(second) = parsers::desc_line(line_set[2], "+")? {
                                (first == second)
                                .then_some(first)
                                .ok_or_else(|| TXaseError::InternalParseFailure(format!("Line three contained a description that did not match the original description!\nExpected: {first:?}, Got: {second:?}")))
                            } else {
                                Ok(first)
                            }
                        )?;
                    Ok((
                        desc.to_string(),
                        line_set[1]
                            .chars()
                            .zip(
                                line_set[3]
                                    .chars()
                            ).map(|(s, q)| -> TXaseResult<_> {
                                Ok((S::Inner::try_from(s)?, Q::try_from(q)?))
                            })
                            .collect::<TXaseResult<_>>()?,
                    ))
                },
            )
            .collect::<Result<HashMap<Descriptor, QualitySequence<S::Inner, Q>>, TXaseError>>()?;
        Ok(Self { sequences })
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
        let sequences = LineSets::new(src)
            .into_par_iter()
            .map(
                |line_set| -> Result<(Descriptor, QualitySequence<S::Inner, Q>), TXaseError> {
                    let desc = parsers::desc_line(line_set[0], "@")?
                        .ok_or_else(|| {
                            TXaseError::InternalParseFailure(
                                "First description line must not be empty!"
                            .to_string())
                        })
                        .and_then(
                            |first| if let Some(second) = parsers::desc_line(line_set[2], "+")? {
                                (first == second)
                                .then_some(first)
                                .ok_or_else(|| TXaseError::InternalParseFailure(format!("Line three contained a description that did not match the original description!\nExpected: {first:?}, Got: {second:?}")))
                            } else {
                                Ok(first)
                            }
                        )?;
                    Ok((
                        desc.to_string(),
                        line_set[1].as_bytes().into_par_iter()
                            .zip(
                                line_set[3].as_bytes().into_par_iter()
                            )
                            .map(|(s, q)| -> TXaseResult<_> {
                                Ok((S::Inner::try_from(char::from(*s))?, Q::try_from(char::from(*q))?))
                            })
                            .collect::<TXaseResult<_>>()?,
                    ))
                },
            )
            .collect::<Result<HashMap<Descriptor, QualitySequence<S::Inner, Q>>, TXaseError>>()?;
        Ok(Self { sequences })
    }
}

struct LineSets<'a> {
    source: Lines<'a>,
}

impl<'a> LineSets<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.lines(),
        }
    }
}

impl<'a> Iterator for LineSets<'a> {
    type Item = [&'a str; 4];

    fn next(&mut self) -> Option<Self::Item> {
        Some([
            self.source.next()?,
            self.source.next()?,
            self.source.next()?,
            self.source.next()?,
        ])
    }
}

#[cfg(feature = "rayon")]
impl<'a> IntoParallelIterator for LineSets<'a> {
    type Iter = rayon::vec::IntoIter<[&'a str; 4]>;

    type Item = [&'a str; 4];

    fn into_par_iter(self) -> Self::Iter {
        self.collect::<Vec<_>>().into_par_iter()
    }
}

mod parsers {
    use nom::{bytes::complete::tag, Parser};

    use crate::err::TXaseResult;

    #[tracing::instrument(level = "trace")]
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
