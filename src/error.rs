use thiserror::Error;

pub type PostalResult<T> = Result<T, PostalError>;

#[derive(Debug, Error)]
pub enum PostalError {
    #[error("could not parse `{0}` as a {1}")]
    ParsingFailure(String, &'static str),
}
