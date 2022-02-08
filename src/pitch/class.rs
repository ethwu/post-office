use std::fmt;

use num::{cast::AsPrimitive, PrimInt};
use pest::{iterators::Pair, Parser};
use phf::{phf_map, phf_ordered_map};

use crate::{error::PostalError, parser::Rule, ExpressionParser, PostalResult};

/// Trait alias for types that can be cast to integer pitch classes.
pub trait IntegerPitchClass = PrimInt + AsPrimitive<isize>;

/// A `PitchClass` corresponds to all notes with the same name, regardless of
/// octave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchClass {
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

// Mapping from note names to pitch classes.
static NOTE_NAMES: phf::Map<&'static str, PitchClass> = phf_map! {
    "C" => PitchClass::C,
    "D" => PitchClass::D,
    "E" => PitchClass::E,
    "F" => PitchClass::F,
    "G" => PitchClass::G,
    "A" => PitchClass::A,
    "B" => PitchClass::B,
    "c" => PitchClass::C,
    "d" => PitchClass::D,
    "e" => PitchClass::E,
    "f" => PitchClass::F,
    "g" => PitchClass::G,
    "a" => PitchClass::A,
    "b" => PitchClass::B,
};

// Mapping from accidentals to their values in semitones.
const ACCIDENTALS: phf::Map<&'static str, i8> = phf_map! {
    "𝄫" => -2,
    "♭" => -1,
    "b" => -1,
    "♮" => 0,
    "n" => 0,
    "♯" => 1,
    "s" => 1,
    "#" => 1,
    "𝄪" => 2,
    "x" => 2,
};

// Mapping from pitch classes to their numerals.
const NUMERALS: [&'static str; 12] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "↊", "↋"];
// Mapping of transdecimal numerals used for parsing.
const TRANSDECIMAL_NUMERALS: phf::OrderedMap<&'static str, PitchClass> = phf_ordered_map! {
    "t" => PitchClass::Bb,
    "T" => PitchClass::Bb,
    "e" => PitchClass::B,
    "E" => PitchClass::B,
};

impl PitchClass {
    /// Get the pitch class from integer notation.
    #[inline]
    pub fn from_int<I: IntegerPitchClass>(i: I) -> Self {
        match i.as_().rem_euclid(12) {
            0 => Self::C,
            1 => Self::Db,
            2 => Self::D,
            3 => Self::Eb,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Gb,
            7 => Self::G,
            8 => Self::Ab,
            9 => Self::A,
            10 => Self::Bb,
            11 => Self::B,
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! pc {
    ($e:expr) => {
        crate::pitch::class::PitchClass::from($e)
    };
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NUMERALS[*self as usize])
    }
}

mod conversions {
    use super::*;

    mod string {
        use std::str::FromStr;

        use lazy_static::lazy_static;
        use lexical::{NumberFormatBuilder, ParseIntegerOptions};

        use super::*;

        const PITCH_CLASS: u128 = NumberFormatBuilder::new().radix(12).build();
        lazy_static! {
            static ref LEXICAL_OPTIONS: ParseIntegerOptions =
                ParseIntegerOptions::builder().build().unwrap();
        }

        /// Parse a `pest` pair into a pitch class. The pair must be a `note_strict`
        /// or a `note_permissive` rule.
        fn parse_note(p: Pair<Rule>) -> PostalResult<PitchClass> {
            let strict = p.as_rule() == Rule::note_strict;
            assert!(strict || p.as_rule() == Rule::note_permissive);

            let mut p = p.into_inner();
            let note_name = p.next().unwrap();

            if let Some(note_name) = NOTE_NAMES.get(&note_name.as_str()) {
                let accidentals: i8 = p
                    .map(|p| ACCIDENTALS.get(p.as_str()).copied().unwrap_or_default())
                    .sum();

                Ok(*note_name + accidentals)
            } else {
                Err(PostalError::ParsingFailure(
                    note_name.as_str().to_string(),
                    "note name",
                ))
            }
        }

        /// Parse a string representing an integer pitch class.
        fn parse_integer(s: &str, strict: bool) -> anyhow::Result<PitchClass> {
            if strict {
                for (i, digit) in NUMERALS.iter().enumerate() {
                    if s == *digit {
                        return Ok(PitchClass::from_int(i));
                    }
                }
                TRANSDECIMAL_NUMERALS
                    .get(s)
                    .copied()
                    .ok_or(PostalError::ParsingFailure(s.to_string(), "integer pitch class").into())
            } else {
                Ok(PitchClass::from_int(lexical::parse_with_options::<
                    isize,
                    _,
                    PITCH_CLASS,
                >(s, &LEXICAL_OPTIONS)?))
            }
        }

        impl FromStr for PitchClass {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut pairs = ExpressionParser::parse(Rule::pitch_class, s)?;
                let mut pairs = pairs.next().unwrap().into_inner();
                let p = pairs.next();
                if let Some(p) = p {
                    Ok(match p.as_rule() {
                        Rule::integer_permissive => parse_integer(p.as_str(), false)?,
                        Rule::integer_strict => parse_integer(p.as_str(), true)?,
                        Rule::note_permissive | Rule::note_strict => parse_note(p)?,
                        _ => unreachable!(),
                    })
                } else {
                    Err(PostalError::ParsingFailure(s.to_string(), "pitch class").into())
                }
            }
        }
    }

    mod integer {
        use num::FromPrimitive;

        use super::*;

        impl FromPrimitive for PitchClass {
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_isize(n: isize) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_i8(n: i8) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_i16(n: i16) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_i32(n: i32) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_i128(n: i128) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_usize(n: usize) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_u8(n: u8) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_u16(n: u16) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_u32(n: u32) -> Option<Self> {
                Some(Self::from_int(n))
            }

            #[inline]
            fn from_u128(n: u128) -> Option<Self> {
                Some(Self::from_int(n))
            }

            fn from_f32(n: f32) -> Option<Self> {
                if n.fract() == 0.0 && n.is_finite() {
                    unsafe { Some(Self::from_int(n.to_int_unchecked::<i32>())) }
                } else {
                    None
                }
            }

            fn from_f64(n: f64) -> Option<Self> {
                if n.fract() == 0.0 && n.is_finite() {
                    unsafe { Some(Self::from_int(n.to_int_unchecked::<i64>())) }
                } else {
                    None
                }
            }
        }

        impl From<i64> for PitchClass {
            #[inline]
            fn from(n: i64) -> Self {
                Self::from_int(n)
            }
        }

        impl From<u64> for PitchClass {
            #[inline]
            fn from(n: u64) -> Self {
                Self::from_int(n)
            }
        }

        impl From<isize> for PitchClass {
            #[inline]
            fn from(n: isize) -> Self {
                Self::from_int(n)
            }
        }

        impl From<i8> for PitchClass {
            #[inline]
            fn from(n: i8) -> Self {
                Self::from_int(n)
            }
        }

        impl From<i16> for PitchClass {
            #[inline]
            fn from(n: i16) -> Self {
                Self::from_int(n)
            }
        }

        impl From<i32> for PitchClass {
            #[inline]
            fn from(n: i32) -> Self {
                Self::from_int(n)
            }
        }

        impl From<usize> for PitchClass {
            #[inline]
            fn from(n: usize) -> Self {
                Self::from_int(n)
            }
        }

        impl From<u8> for PitchClass {
            #[inline]
            fn from(n: u8) -> Self {
                Self::from_int(n)
            }
        }

        impl From<u16> for PitchClass {
            #[inline]
            fn from(n: u16) -> Self {
                Self::from_int(n)
            }
        }

        impl From<u32> for PitchClass {
            #[inline]
            fn from(n: u32) -> Self {
                Self::from_int(n)
            }
        }
    }
}

mod operations {
    use std::ops;

    use super::*;

    impl<I: IntegerPitchClass> ops::Add<I> for PitchClass {
        type Output = PitchClass;

        fn add(self, rhs: I) -> Self::Output {
            Self::from(self as isize + rhs.as_())
        }
    }

    impl<I: IntegerPitchClass> ops::Sub<I> for PitchClass {
        type Output = PitchClass;

        fn sub(self, rhs: I) -> Self::Output {
            Self::from(self as isize - rhs.as_())
        }
    }
}

#[cfg(test)]
mod test {
    use assert2::assert;

    use super::*;

    /// Check that integers are correctly cast into pitch classes.
    #[test]
    fn int_to_pc() {
        assert!(PitchClass::D == 14u8.into());
        assert!(PitchClass::Bb == (-2i32).into());
        assert!(PitchClass::C == 24.into());
    }

    /// Check that the `pc!` macro functions correctly.
    #[test]
    fn pc_macro() {
        assert!(pc!(8) == PitchClass::Ab);
        assert!(pc!(12) == PitchClass::C);
        assert!(pc!(-145) == PitchClass::B);
    }

    /// Check that note names are cast correctly.
    #[test]
    fn note_names() {
        use std::str::FromStr;

        for (s, pc) in [
            ("c", PitchClass::C),
            ("a♭", PitchClass::Ab),
            ("Abx#b", PitchClass::Bb),
            ("Fx", PitchClass::G),
            ("bbb", PitchClass::A),
            ("-8", PitchClass::E),
            ("F♭b", PitchClass::Eb),
        ] {
            assert!(
                PitchClass::from_str(s).is_ok(),
                "string: {:?}; pitch class: {:?}",
                s,
                pc,
            );
            assert!(
                PitchClass::from_str(s).unwrap() == pc,
                "string: {:?}; pitch class: {:?}",
                s,
                pc,
            );
        }

        for s in ["", "♭bb"] {
            assert!(PitchClass::from_str(s).is_err(), "string: {:?}", s);
        }
    }

    /// Check the display of pitch classes.
    #[test]
    fn display() {
        for (pc, s) in [
            (-1, "↋"),
            (0, "0"),
            (3, "3"),
            (11, "↋"),
            (12, "0"),
            (130, "↊"),
        ] {
            assert!(format!("{}", pc!(pc)) == s, "pitch class: {:?}", pc);
        }
    }
}
