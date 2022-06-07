#![warn(missing_debug_implementations)]
#![allow(clippy::missing_errors_doc)]

pub mod err;
pub mod fasta;
pub mod fastq;
pub mod genomics;
pub mod gff;
pub mod proteomics;

pub(crate) mod util {
    #[derive(Debug)]
    pub(crate) struct Memchr1Split<'a> {
        source: Option<&'a [u8]>,
        needle: u8,
    }

    impl<'a> Memchr1Split<'a> {
        pub(crate) fn new(needle: u8, haystack: &str) -> Memchr1Split<'_> {
            let source = Some(haystack.as_bytes());
            Memchr1Split { source, needle }
        }
    }

    impl<'a> Iterator for Memchr1Split<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            let source = self.source?;
            if let Some(pos) = memchr::memchr(self.needle, source) {
                let (ret, rem) = source.split_at(pos);
                self.source = Some(rem);
                Some(
                    std::str::from_utf8(ret)
                        .expect("self.source was a valid str when self was constructed"),
                )
            } else {
                self.source
                    .map(std::str::from_utf8)
                    .take()
                    .transpose()
                    .expect("self.source was a valid str when self was constructed")
            }
        }
    }
    #[derive(Debug)]
    pub(crate) struct Memchr2Split<'a> {
        source: Option<&'a [u8]>,
        needles: (u8, u8),
    }

    impl<'a> Memchr2Split<'a> {
        pub(crate) fn new(needle1: u8, needle2: u8, haystack: &str) -> Memchr2Split<'_> {
            let source = Some(haystack.as_bytes());
            Memchr2Split {
                source,
                needles: (needle1, needle2),
            }
        }
    }

    impl<'a> Iterator for Memchr2Split<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            let source = self.source?;
            if let Some(pos) = memchr::memchr2(self.needles.0, self.needles.1, source) {
                let (ret, rem) = source.split_at(pos);
                self.source = Some(rem);
                Some(
                    std::str::from_utf8(ret)
                        .expect("self.source was a valid str when self was constructed"),
                )
            } else {
                self.source
                    .map(std::str::from_utf8)
                    .take()
                    .transpose()
                    .expect("self.source was a valid str when self was constructed")
            }
        }
    }
    #[derive(Debug)]
    pub(crate) struct Memchr3Split<'a> {
        source: Option<&'a [u8]>,
        needles: (u8, u8, u8),
    }

    impl<'a> Memchr3Split<'a> {
        pub(crate) fn new(
            needle1: u8,
            needle2: u8,
            needle3: u8,
            haystack: &str,
        ) -> Memchr3Split<'_> {
            let source = Some(haystack.as_bytes());
            Memchr3Split {
                source,
                needles: (needle1, needle2, needle3),
            }
        }
    }

    impl<'a> Iterator for Memchr3Split<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<Self::Item> {
            let source = self.source?;
            if let Some(pos) =
                memchr::memchr3(self.needles.0, self.needles.1, self.needles.2, source)
            {
                let (ret, rem) = source.split_at(pos);
                self.source = Some(rem);
                Some(
                    std::str::from_utf8(ret)
                        .expect("self.source was a valid str when self was constructed"),
                )
            } else {
                self.source
                    .map(std::str::from_utf8)
                    .take()
                    .transpose()
                    .expect("self.source was a valid str when self was constructed")
            }
        }
    }
}
#[cfg(test)]
mod tests {}
