//! TODO
//!
//! # Author
//! Pierre-Louis GAUTIER

#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, $($arg:tt)+) => {{ TRACE!("{} >> {}", $tp, format_args!($($arg)+)); }};
    (type=>$tp:expr) => {{ TRACE!("{}", $tp); }};
    ($($arg:tt)+) => {{
        #[cfg(debug_assertions)]
        println!("{}", format_args!("{}:{} - {}", file!(), line!(), format_args!($($arg)+)));
        #[cfg(not(debug_assertions))]
        {} // TODO write in FILE
}};
}

#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),+) => {
        { TRACE!(type => "DEBUG", $($msg),+); }
    };
}

#[macro_export]
macro_rules! INFO {
    ($($msg:expr),+) => {{ TRACE!(type => "INFO", $($msg),+); }};
}

#[macro_export]
macro_rules! WARNING {
    ($($msg:expr),+) => {{ TRACE!(type => "WARNING", $($msg),+); }};
}

#[macro_export]
macro_rules! ERROR {
    ($($msg:expr),+) => {{ TRACE!(type => "ERROR", $($msg),+); }};
}
