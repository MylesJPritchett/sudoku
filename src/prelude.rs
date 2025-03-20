//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple strcut for newtype pattern
pub struct W<T>(pub T);

pub use crate::puzzle::import::*;
pub use crate::puzzle::solve::*;
pub use crate::puzzle::*;
