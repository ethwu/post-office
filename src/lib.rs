#![feature(min_specialization)]
#![feature(trait_alias)]

pub mod error;
mod parser;
pub mod pitch;

pub use self::error::*;
pub use self::parser::*;
pub use self::pitch::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
