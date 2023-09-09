#[allow(clippy::needless_doctest_main)]
pub(crate) mod extra;
mod func;
mod matrix;

/// modules for calculating Special Functions (Gamma, Beta, Error, etc.)
pub mod special;

/// modules for calculating Statistical Data
pub mod stats;

pub use func::*;
pub use matrix::*;
