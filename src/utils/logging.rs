use once_cell::sync::Lazy;
use std::sync::Mutex;

//#region level

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Verbose3,
    Verbose2,
    Verbose,
    Info,
    Warn,
    Error,
}

static G_LOG_LEVEL: Lazy<Mutex<LogLevel>> = Lazy::new(|| Mutex::new(LogLevel::Info));

pub fn set_log_level(level: LogLevel) {
    let mut mode = G_LOG_LEVEL.lock().unwrap();
    *mode = level;
}

pub fn get_log_level() -> LogLevel {
    let mode = G_LOG_LEVEL.lock().unwrap();
    return *mode;
}

#[macro_export]
macro_rules! check_log_level {
    (::$item:ident) => {{ crate::logging::get_log_level() <= crate::logging::LogLevel::$item }};
}

//#endregion
//#region log!

#[macro_export]
macro_rules! _log {
    (::$item:ident, $($arg:tt)*) => {{
        if check_log_level!(::$item) {
            println!($($arg)*);
        }
    }};
}
#[macro_export]
macro_rules! _elog {
    (::$item:ident, $($arg:tt)*) => {{
        if check_log_level!(::$item) {
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => {{ _elog!(::Error, $($arg)*) }};
}

#[macro_export]
macro_rules! wlog {
    ($($arg:tt)*) => {{ _log!(::Warn, $($arg)*) }};
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{ _log!(::Info, $($arg)*) }};
}

#[macro_export]
macro_rules! vlog {
    ($($arg:tt)*) => {{ _log!(::Verbose, $($arg)*) }};
}
#[macro_export]
macro_rules! vvlog {
    ($($arg:tt)*) => {{ _log!(::Verbose2, $($arg)*) }};
}
#[macro_export]
macro_rules! vvvlog {
    ($($arg:tt)*) => {{ _log!(::Verbose3, $($arg)*) }};
}

//#endregion
//#region dbgx!

pub fn _dbgx<T: std::fmt::Debug>(val: &T, indent: usize) {
    let s = format!("{:#?}", val);
    eprintln!("{}", s.replace("    ", &" ".repeat(indent)));
}
#[macro_export]
macro_rules! dbgx {
    ($e:expr) => {{ crate::logging::_dbgx($e, 2) }};
}

//#endregion
