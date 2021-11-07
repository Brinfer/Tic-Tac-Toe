//! Set of macro to have [TRACE], or log during the execution of the program.
//!
//! There are different levels:
//! - [`TRACE`]: for trace message
//! - [`DEBUG`]: for debug message
//! - [`INFO`]: for informative message
//! - [`WARNING`]: for warning message
//! - [`ERROR`]: for error message
//!
//! The format of the messages will be : `file:line - LEVEL >> message`
//!
//! # Example
//!
//! ```rust
//! use crate::{TRACE, DEBUG, INFO, WARNING, ERROR};
//!
//! TRACE!("HelloTrace");
//! DEBUG!("HelloDebug")
//! INFO!("HelloInfo");
//! WARNING!("HelloWarning");
//! ERROR!("HelloError");
//!
//! for i 0..2 {
//!     TRACE("Trace message n° {}", i)
//! }
//! ```
//!
//! # Disadvantage
//!
//! The use of function instead of macro would have brought more flexibility in the use of this module.
//! Indeed, as each macro ([`DEBUG`], [`INFO`],...) comes back to [`TRACE`], it is necessary to import [`TRACE`] to be able to use the others.
//!
//! The choice was made to keep these macros in spite of everything, rather to be able to try and see how the macro system works in __Rust__
//!
//! # Task lists
//!
//! - [x] Create various macro
//! - [x] Be able to write any type of message
//! - [ ] Write the log to a file
//!
//! # Author
//!
//! Pierre-Louis GAUTIER

#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, $($arg:tt)+) => {{ TRACE!("{} >> {}", $tp, format_args!($($arg)+)) }};
    (type=>$tp:expr) => {{ TRACE!("{}", $tp) }};
    ($($arg:tt)+) => {{
        #[cfg(debug_assertions)]
        println!("{}", format_args!("{}:{} - {}", file!(), line!(), format_args!($($arg)+)));
        #[cfg(not(debug_assertions))]
        {} // TODO write in FILE
}};
}

#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),+) => {{ TRACE!(type => "DEBUG", $($msg),+) }};
}

#[macro_export]
macro_rules! INFO {
    ($($msg:expr),+) => {{ TRACE!(type => "INFO", $($msg),+) }};
}

#[macro_export]
macro_rules! WARNING {
    ($($msg:expr),+) => {{ TRACE!(type => "WARNING", $($msg),+) }};
}

#[macro_export]
macro_rules! ERROR {
    ($($msg:expr),+) => {{ TRACE!(type => "ERROR", $($msg),+) }};
}
