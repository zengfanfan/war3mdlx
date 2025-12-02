use crate::*;

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

impl Args {
    pub fn set_log_level(level: LogLevel) {
        let mut v = G_LOG_LEVEL.lock().unwrap();
        *v = level;
    }
    pub fn get_log_level() -> LogLevel {
        let v = G_LOG_LEVEL.lock().unwrap();
        return *v;
    }
}

#[macro_export]
macro_rules! check_log_level {
    (::$item:ident) => {{ crate::cli::Args::get_log_level() <= crate::logging::LogLevel::$item }};
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

pub fn _dbgx<T: stdDebug>(val: &T, indent: usize) {
    let s = format!("{:#?}", val);
    vvvlog!("{}", s.replace("    ", &" ".repeat(indent)));
}
#[macro_export]
macro_rules! dbgx {
    ($e:expr) => {{ crate::logging::_dbgx($e, 2) }};
}

//#endregion
