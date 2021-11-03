#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, $($arg:tt)+) => {{ TRACE!("{} >> {}", $tp, format_args!($($arg)+)); }};
    (type=>$tp:expr) => {{ TRACE!("{}", $tp); }};
    ($($arg:tt)+) => {{ println!("{}", format_args!("{}:{} - {}", file!(), line!(), format_args!($($arg)+))); }};
}

#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),+) => {
        #[cfg(debug_assertions)]
        { TRACE!(type => "DEBUG", $($msg),+); }
        #[cfg(not(debug_assertions))]
            {}
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
