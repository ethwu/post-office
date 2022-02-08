pub mod class;

use std::{cmp::Ordering, str::FromStr};

use anyhow::{Error, Result};
use pest::Parser;

pub use self::class::*;
use crate::{ExpressionParser, PostalError, Rule};

/// An octave specifies how high or low a pitch is.
pub type Octave = i8;

/// A pitch consists of a pair of pitch class and octave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    /// This note's pitch class.
    class: PitchClass,
    /// This note's octave.
    octave: Octave,
}

impl Pitch {
    pub fn new<PC: Into<PitchClass>>(pc: PC, oct: Octave) -> Self {
        Self {
            class: pc.into(),
            octave: oct,
        }
    }
}

impl PartialOrd<Self> for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.octave.partial_cmp(&other.octave) {
            Some(Ordering::Equal) => (self.class as isize).partial_cmp(&(other.class as isize)),
            ord => ord,
        }
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Pitch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let p = ExpressionParser::parse(Rule::pitch, s)?.next();

        if let Some(p) = p {
            assert_eq!(p.as_rule(), Rule::pitch);

            let mut p = p.into_inner();
            let note = p.next().unwrap();
            let octave = p.next().unwrap();

            assert_eq!(note.as_rule(), Rule::note_permissive);
            assert_eq!(octave.as_rule(), Rule::octave);

            todo!()
        } else {
            Err(PostalError::ParsingFailure(s.to_string(), "pitch").into())
        }
    }
}

#[cfg(test)]
mod test {
    use assert2::assert;

    use super::*;

    #[test]
    fn pitch_comparison() {
        assert!(Pitch::new(PitchClass::F, 3) == Pitch::new(PitchClass::F, 3));
        assert!(Pitch::new(PitchClass::C, 4) < Pitch::new(PitchClass::F, 4));
        assert!(Pitch::new(PitchClass::G, 2) < Pitch::new(PitchClass::C, 4));
        assert!(Pitch::new(PitchClass::A, 6) > Pitch::new(PitchClass::F, 4));
    }
}
