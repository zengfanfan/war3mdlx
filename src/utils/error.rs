use crate::*;

#[derive(Debug)]
pub enum MyError {
    String(String),
    Io(ioError),
}

//#region trait: Display

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::String(s) => write!(f, "{}", s),
            MyError::Io(e) => write!(f, "{}", e),
        }
    }
}

//#endregion
//#region trait: Convert From

impl From<String> for MyError {
    fn from(e: String) -> Self {
        MyError::String(e)
    }
}
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

//#endregion

#[macro_export]
macro_rules! ERR {
    ($($arg:tt)*) => {{
        core::result::Result::Err(crate::error::MyError::String(format!($($arg)*)))
    }};
}
