//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // For beginning only

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
