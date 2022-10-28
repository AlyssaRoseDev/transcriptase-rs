/// Defines the requirements for an ASCII character encoded quality score for FastQ files
pub trait Quality: From<f64> + Into<f64> + TryFrom<char, Error = String> + Into<char> {
    fn is_valid(c: char) -> bool {
        ('!'..='~').contains(&c)
    }
}

#[cfg(test)]
#[test]
fn contains() {
    for c in r##"!"#$%&'()*+,./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~`-"##.chars() {
        assert!(Phred::is_valid(c));
        assert!(Solexa::is_valid(c));
    }
}

/// A Phred quality score, encoded by the formula `-10 * log10(P)` for a probability `P`
#[derive(Debug)]
pub struct Phred(u8);

impl From<Phred> for f64 {
    fn from(code: Phred) -> Self {
        10.0_f64.powf(code.0 as f64 / -10.0)
    }
}

impl From<f64> for Phred {
    fn from(score: f64) -> Self {
        Self((f64::log10(score) * -10.0) as u8)
    }
}

impl TryFrom<char> for Phred {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let byte = value as u8;
        if !(0x21..=0x7e).contains(&byte) {
            Err(format!(
                "Phred quality character must be between '!' (0x21) and '~' (0x7e), got {value} ({byte:x})"
            ))
        } else {
            Ok(Self(byte - 0x21))
        }
    }
}

impl From<Phred> for char {
    fn from(score: Phred) -> Self {
        (score.0 + 0x21) as char
    }
}

impl Quality for Phred {}

/// A Quality score from pre-1.3 versions of the Solexa pipeline, encoded by the formula `-10 * log10(p / 1-p)`
#[derive(Debug)]
pub struct Solexa(u8);

impl From<Solexa> for f64 {
    fn from(code: Solexa) -> Self {
        let code = code.0 as f64;
        let partial = 10.0_f64.powf(code / -10.0_f64);
        partial / (1.0_f64 + partial)
    }
}

impl From<f64> for Solexa {
    fn from(prob: f64) -> Self {
        Solexa((-10.0_f64 * f64::log10(prob / (1.0 - prob))) as u8)
    }
}

impl TryFrom<char> for Solexa {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let byte = value as u8;
        if !(0x21..=0x7e).contains(&byte) {
            Err(format!(
                r#"Solexa quality character must be between '!' (0x21) and '~' (0x7e), got {value} ({byte:x})"#
            ))
        } else {
            Ok(Self(byte))
        }
    }
}

impl From<Solexa> for char {
    fn from(score: Solexa) -> Self {
        score.0 as char
    }
}

impl Quality for Solexa {}
