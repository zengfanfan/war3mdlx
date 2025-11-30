use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Verbose,
    Info,
    // Warn,
    Error,
}

static G_LOG_LEVEL: Lazy<Mutex<LogLevel>> = Lazy::new(|| Mutex::new(LogLevel::Info));

pub fn set_level(level: LogLevel) {
    let mut mode = G_LOG_LEVEL.lock().unwrap();
    *mode = level;
}

pub fn get_level() -> LogLevel {
    let mode = G_LOG_LEVEL.lock().unwrap();
    return *mode;
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        if logging::get_level() <= logging::LogLevel::Info {
            println!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! vlog {
    ($($arg:tt)*) => {{
        if logging::get_level() == logging::LogLevel::Verbose {
            println!($($arg)*);
        }
    }};
}
