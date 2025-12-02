use crate::*;

pub trait Extend_Path {
    fn ext(&self) -> &str;
    fn ext_lower(&self) -> String;
    fn ext_upper(&self) -> String;
}

impl Extend_Path for Path {
    fn ext(&self) -> &str {
        self.extension().and_then(|s| s.to_str()).unwrap_or("")
    }
    fn ext_lower(&self) -> String {
        self.ext().to_lowercase()
    }
    fn ext_upper(&self) -> String {
        self.ext().to_uppercase()
    }
}
