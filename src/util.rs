#[derive(Debug)]
pub(crate) struct Split1Iter<'a> {
    source: Option<&'a [u8]>,
    needle: u8,
    size: usize,
}

impl<'a> Split1Iter<'a> {
    pub(crate) fn new(needle: u8, haystack: &str) -> Split1Iter<'_> {
        let source = Some(haystack.as_bytes());
        let size = haystack
            .bytes()
            .fold(0, |count, byte| count + (byte == needle) as usize);
        Split1Iter {
            source,
            needle,
            size,
        }
    }
}

impl<'a> Iterator for Split1Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source?;
        if let Some(pos) = memchr::memchr(self.needle, &source[1..]) {
            let (ret, rem) = source.split_at(pos);
            self.source = Some(rem);
            self.size -= 1;
            Some(
                std::str::from_utf8(ret)
                    .expect("self.source was a valid str when self was constructed"),
            )
        } else {
            self.size -= 1;
            self.source
                .map(std::str::from_utf8)
                .take()
                .transpose()
                .expect("self.source was a valid str when self was constructed")
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

#[cfg(feature = "rayon")]
impl<'a> rayon::iter::IntoParallelIterator for Split1Iter<'a> {
    type Iter = rayon::vec::IntoIter<&'a str>;

    type Item = &'a str;

    fn into_par_iter(self) -> Self::Iter {
        self.collect::<Vec<_>>().into_par_iter()
    }
}

#[derive(Debug)]
pub(crate) struct Split2Iter<'a> {
    source: Option<&'a [u8]>,
    needles: (u8, u8),
    size: usize,
}

impl<'a> Split2Iter<'a> {
    pub(crate) fn new(needle1: u8, needle2: u8, haystack: &str) -> Split2Iter<'_> {
        let source = Some(haystack.as_bytes());
        let size = haystack.bytes().fold(0, |count, byte| {
            count + (byte == needle1 || byte == needle2) as usize
        });
        Split2Iter {
            source,
            needles: (needle1, needle2),
            size,
        }
    }
}

impl<'a> Iterator for Split2Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source?;
        if let Some(pos) = memchr::memchr2(self.needles.0, self.needles.1, &source[1..]) {
            let (ret, rem) = source.split_at(pos);
            self.source = Some(rem);
            self.size -= 1;
            Some(
                std::str::from_utf8(ret)
                    .expect("self.source was a valid str when self was constructed"),
            )
        } else {
            self.size -= 1;
            self.source
                .map(std::str::from_utf8)
                .take()
                .transpose()
                .expect("self.source was a valid str when self was constructed")
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

#[cfg(feature = "rayon")]
impl<'a> rayon::iter::IntoParallelIterator for Split2Iter<'a> {
    type Iter = rayon::vec::IntoIter<&'a str>;

    type Item = &'a str;

    fn into_par_iter(self) -> Self::Iter {
        self.collect::<Vec<_>>().into_par_iter()
    }
}

#[derive(Debug)]
pub(crate) struct Split3Iter<'a> {
    source: Option<&'a [u8]>,
    needles: (u8, u8, u8),
    size: usize,
}

impl<'a> Split3Iter<'a> {
    pub(crate) fn new(needle1: u8, needle2: u8, needle3: u8, haystack: &str) -> Split3Iter<'_> {
        let source = Some(haystack.as_bytes());
        let size = haystack.bytes().fold(0, |count, byte| {
            count + (byte == needle1 || byte == needle2 || byte == needle3) as usize
        });
        Split3Iter {
            source,
            needles: (needle1, needle2, needle3),
            size,
        }
    }
}

impl<'a> Iterator for Split3Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source?;
        if let Some(pos) =
            memchr::memchr3(self.needles.0, self.needles.1, self.needles.2, &source[1..])
        {
            let (ret, rem) = source.split_at(pos);
            self.source = Some(rem);
            self.size -= 1;
            Some(
                std::str::from_utf8(ret)
                    .expect("self.source was a valid str when self was constructed"),
            )
        } else {
            self.size -= 1;
            self.source
                .map(std::str::from_utf8)
                .take()
                .transpose()
                .expect("self.source was a valid str when self was constructed")
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

#[cfg(feature = "rayon")]
impl<'a> rayon::iter::IntoParallelIterator for Split3Iter<'a> {
    type Iter = rayon::vec::IntoIter<&'a str>;

    type Item = &'a str;

    fn into_par_iter(self) -> Self::Iter {
        self.collect::<Vec<_>>().into_par_iter()
    }
}
