//! This is an opinionated CLI library for beautiful and user-friendly terminal
//! output. It aims to give the Rust ecosystem similar tooling as
//! [Rich](https://github.com/Textualize/rich) for Python.
//!
//! Note that the library is still very much a work in progress.
//!
//! # Available utilies
//! - [CLI messaging](msg)

pub mod msg;

#[doc(hidden)]
pub use colored;
