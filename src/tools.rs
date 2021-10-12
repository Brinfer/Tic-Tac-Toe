#[derive(Debug)]
pub enum PlayerRole {
    UNKNOWN = 0,
    HOST,
    CLIENT,
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        {
            println!("Info >> {}", $msg);
        }
        #[cfg(not(debug_assertions))]
        {}
    };
}
