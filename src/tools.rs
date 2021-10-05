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
