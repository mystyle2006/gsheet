#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("\x1b[36m[DEBUG] {}\x1b[0m", format!($($arg)*));
        }
    };
}