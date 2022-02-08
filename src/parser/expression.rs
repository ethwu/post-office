use std::{fmt, str::FromStr};

use anyhow::Result;
use pest::{iterators::Pair, Parser};

use crate::{
    parser::{ExpressionParser, Rule},
    Pitch, PitchClass,
};

/// A complete expression.
#[derive(Debug)]
pub enum Expression {
    Pitch(Pitch),
    PitchClass(PitchClass),
    Collection,
}

impl Expression {
    pub fn from_str(e: &str) -> Result<Self> {
        log::debug!("Parsing string '{}' as an expression.", e);
        let mut pairs = ExpressionParser::parse(Rule::expression, e)?;

        Ok(Self::from_pair(pairs.next().unwrap()))
    }

    fn from_pair(p: Pair<Rule>) -> Self {
        log::trace!("Parsing pair `{}`.", p);
        match p.as_rule() {
            Rule::expression => Self::from_pair(p.into_inner().next().unwrap()),
            Rule::pitch_class_permissive | Rule::pitch_class_strict => Expression::PitchClass(
                PitchClass::from_str(p.into_inner().next().unwrap().as_str()).unwrap(),
            ),
            _ => todo!(),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pitch(p) => write!(f, "{:?}", p),
            Self::PitchClass(pc) => pc.fmt(f),
            Self::Collection => write!(f, "{{unimplemented}}"),
        }
    }
}
