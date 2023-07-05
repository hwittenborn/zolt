//! Utilities for general CLI messaging.
//!
//! It's recommended to call these macros from the root of the crate instead of
//! calling them from here. I.e. call [`crate::info!`] instead of
//! [`crate::msg::info!`].
//!
//! # Message types
//! There are four messaging types available:
//! - Info: [`info!`], [`infoln!`], [`infofmt!`]
//! - Warning: [`warn!`], [`warnln!`], [`warnfmt!`]
//! - Error: [`err!`], [`errln!`], [`errfmt!`]
//! - Debug: [`debug!`], [`debugln!`], [`debugfmt!`]
//!
//! # Macro types
//! The first form of these macros ([`info!`], [`warn!`], [`err!`], [`debug!`])
//! all behave like [`print!`] does - printing to the terminal *without* a
//! newline.
//!
//! The second form of these macros ([`infoln!`], [`warnln!`], [`errln!`],
//! [`debugln!`]) all behave line [`println!`] does - printing to the terminal
//! *with* a newline.
//!
//! The last form of these macros ([`infofmt!`], [`warnfmt!`], [`errfmt!`],
//! [`debugfmt!`]) all behave like [`format!`] does - returning the formatted
//! string directly.
//!
//! # Message behavior
//! The Info macros print to [`std::io::Stdout`], while all the rest print to [`std::io::Stderr`].
//!
//! If this behavior isn't desired, you can obtain the formatted string from the `*fmt` macros and
//! then do whatever you need with them.
//!
//! # Coloring message contents
//! The contents of messages can be colored by using [`colored::Colorize`], which is re-exported
//! under `zolt::Colorize`:
//!
//! ```rust
//! use zolt::Colorize;
//!
//! zolt::infoln!("Here's a message with {} output!", "colored".blue());
//! ```
#[doc(inline)]
pub use zolt_macros::{
    debug, debugfmt, debugln, err, errfmt, errln, info, infofmt, infoln, warn, warnfmt, warnln,
};
