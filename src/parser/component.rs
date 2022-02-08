use pest::iterators::Pair;

use crate::parser::Rule;

/// A component that does not constitute an expression.
#[derive(Debug, Clone, Copy)]
pub enum Component<'c> {
    NoteName(&'c str),
    Accidental(&'c str),
    Octave(isize),
}

impl Component<'_> {
    pub fn from_pair(p: Pair<Rule>) -> Self {
        match p.as_rule() {
            _ => todo!(),
        }
    }
}
