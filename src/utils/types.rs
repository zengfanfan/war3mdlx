pub type byte = u8;

pub type int = i32;
pub type uint = u32;
pub type long = i64;
pub type ulong = u64;

pub type float = f32;
pub type double = f64;

#[macro_export]
macro_rules! yesno {
    ($cond:expr, $y:expr, $n:expr) => {{ if $cond { $y } else { $n } }};
}

#[macro_export]
macro_rules! yes {
    ($cond:expr, $y:stmt) => {{ if $cond { $y } }};
}

#[macro_export]
macro_rules! no {
    ($cond:expr, $n:stmt) => {{ if !($cond) { $n } }};
}
