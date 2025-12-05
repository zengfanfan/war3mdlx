use crate::*;

pub trait _ExtendPath {
    fn ext(&self) -> &str;
    fn ext_lower(&self) -> String;
}

impl _ExtendPath for Path {
    fn ext(&self) -> &str {
        self.extension().and_then(|s| s.to_str()).unwrap_or("")
    }
    fn ext_lower(&self) -> String {
        self.ext().to_lowercase()
    }
}
