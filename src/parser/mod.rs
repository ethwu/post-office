mod component;
mod expression;

use pest_derive::Parser;

pub use self::component::*;
pub use self::expression::*;

#[derive(Debug, Parser)]
#[grammar = "mus.pest"]
pub(crate) struct ExpressionParser;
