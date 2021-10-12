#[derive(Debug, PartialEq, Eq)]
pub enum PlayerRole {
    UNKNOWN = 0,
    HOST,
    GUEST,
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("Info\t>> {}", $msg);
        }
        #[cfg(not(debug_assertions))]
        {}
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        println!("Warning\t>> {}", $msg);
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        println!("Error\t>> {}", $msg);
    };
}
