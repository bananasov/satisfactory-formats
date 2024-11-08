use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid UTF-8: {0}")]
    InvalidUTF8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Scroll(#[from] scroll::Error),
}