#[derive(Debug)]
pub enum MyError {
    String(String),
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Empty(()),
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        MyError::Parse(e)
    }
}
