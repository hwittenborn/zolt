//! Utilities for general CLI messaging.
//!
//! For general messaging needs, use one of the available macros:
//!
//! - [`info!`]
//! - [`warn!`]
//! - [`err!`]
//! - [`debug!`]
//!
//! It's recommended that the items in this module are called from the
//! [crate root](crate#macros):
//!
//! ```rust
//! /// Call the `info` macro like this:
//! zolt::info!("Application started.");
//!
//! /// Call the `warn` macro like this:
//! zolt::warn!("Application is still starting...");
//!
//! /// Call the `err` macro like this:
//! zolt::err!("Application could not start!");
//!
//! /// call the `debug` macro like this:
//! zolt::debug!("Application shutting down...");
//! ```

#[doc(inline)]
pub use crate::{debug, err, info, warn};

/// Print a message suitable for general information.
///
/// It takes the same arguments as the [`format!`] macro.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        println!("{} {}", "Info:".cyan().bold(), format!($($arg)*));
    }}
}

/// Print a message suitable for warning information.
///
/// It takes the same arguments as the [`format!`] macro.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        println!("{} {}", "Warning:".bright_yellow().bold(), format!($($arg)*));
    }}
}

/// Print a message suitable for error information.
///
/// It takes the same arguments as the [`format!`] macro.
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        println!("{} {}", "Error:".red().bold(), format!($($arg)*));
    }}
}

/// Print a message suitable for debug information.
///
/// It takes the same arguments as the [`format!`] macro.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        println!("{} {}", "Debug:".green().bold(), format!($($arg)*));
    }}
}
