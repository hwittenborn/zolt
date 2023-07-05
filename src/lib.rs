//! This is an opinionated CLI library for beautiful and user-friendly terminal
//! output. It aims to give the Rust ecosystem similar tooling as
//! [Rich](https://github.com/Textualize/rich) for Python.
//!
//! Note that the library is still very much a work in progress.
//!
//! # Available utilies
//! - [CLI messaging](msg)
//!
//! # Enabling/disabling color
//! Most of the functions in this library include color in some fashion when
//! printing to the terminal.
//!
//! Color will be automatically disabled in any of these circumstances:
//! - The `NO_COLOR` environment variable is set
//! - The `CLICOLOR` environment variable is set to `0`
//! - [`std::io::Stdout`] isn't a TTY (i.e. when being piped or in a noninteractive terminal)
//!
//! The above situations are ignored in any of these circumstances:
//! - The `CLICOLOR` environment variable is set to anything other than `0`
//! - The `CLICOLOR_FORCE` environment variable is set
//!
//! CLI coloring can also be manually set on and off through the [`show_color`] function.
pub mod msg;

/// Whether coloring should be on or off.
///
/// If this function is never called, coloring behavior is automatically decided
/// (see [Enabling/disabling color](crate#enablingdisabling-color)).
#[inline(always)]
pub fn show_color(show: bool) {
    colored::control::set_override(show);
}

#[doc(no_inline)]
pub use msg::{
    debug, debugfmt, debugln, err, errfmt, errln, info, infofmt, infoln, warn, warnfmt, warnln,
};

#[doc(hidden)]
pub use colored;
