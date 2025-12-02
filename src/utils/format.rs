use crate::*;

pub fn fmtx<T: Formatter>(v: &T) -> String {
    T::debug(v)
}

fn trim_float_str(s: &str) -> String {
    let (i, f) = match s.split_once('.') {
        Some((i, f)) => (i, f),
        None => (s, ""),
    };
    let mut i = i.trim_start_matches(&[' ', '0']);
    let f = f.trim_end_matches(&[' ', '0']);
    if i.len() == 0 {
        i = "0";
    }
    if f.len() == 0 { i.to_string() } else { format!("{}.{}", i, f) }
}

pub fn fmt_float(v: &f32, len: u32, precision: u32) -> String {
    let len = len as usize;
    let precision = precision as usize;
    let s = format!("{:.*}", precision, v);
    let s = trim_float_str(&s);
    if s.len() <= len {
        return s;
    }
    let s = format!("{:.*e}", precision, v);
    let (i, e) = s.split_once('e').unwrap();
    let i = trim_float_str(&i);
    let s = format!("{}e{}", i, e);
    let ev: int = e.parse().unwrap();
    match ev.abs() as usize > (len + precision) / 2 {
        true => s,
        false => {
            let v: f32 = s.parse().unwrap();
            let s = format!("{:.*}", precision, v);
            return trim_float_str(&s);
        },
    }
}

pub fn fmt_id4s(v: &u32) -> String {
    u32_to_ascii(*v)
}

pub fn u32_to_ascii(n: u32) -> String {
    let bytes = n.to_be_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[macro_export]
macro_rules! TNAMEL {
    () => {
        std::any::type_name::<Self>()
    };
    ($t:ty) => {
        std::any::type_name::<$t>()
    };
}

#[macro_export]
macro_rules! TNAME {
    () => {
        tname_last_seg_trim::<Self>(2)
    };
    ($t:ty) => {
        tname_last_seg_trim::<$t>(2)
    };
}

pub fn tname_last_seg_trim<T>(n: u32) -> String {
    let n = n as usize;
    let full = TNAMEL!(T);
    let parts: Vec<&str> = full.split("::").collect();
    let len = parts.len();
    if n >= len { full.to_string() } else { parts[len - n..].join("::") }
}

//#region trait: Formatter

pub trait Formatter: Sized {
    fn debug(&self) -> String;
}

macro_rules! impl_formatter {
    ($($t:ty),*) => {
        $(
            impl Formatter for $t {
                fn debug(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    };
}
macro_rules! impl_formatter_array {
    ($($t:ty),*) => {
        $(
            impl Formatter for Vec<$t> {
                fn debug(&self) -> String {
                    format!("[{}]", self.iter().map(|x| Formatter::debug(x)).collect::<Vec<_>>().join(", "))
                }
            }
            impl Formatter for &[$t] {
                fn debug(&self) -> String {
                    Formatter::debug(&self.to_vec())
                }
            }
        )*
    };
}
macro_rules! impl_formatter_vec234 {
    ($($t:ty),*) => {
        $(
            impl Formatter for $t {
                fn debug(&self) -> String {
                    let s=Formatter::debug(&self.to_array().to_vec());
                    format!("({})", s[1..s.len()-1].to_string())
                }
            }
        )*
    };
}

impl_formatter!(i8, u8, i16, u16, i32, u32);
impl_formatter_array!(i8, u8, i16, u16, i32, u32, f32);
impl_formatter_vec234!(Vec2, Vec3, Vec4);
impl_formatter_array!(Vec2, Vec3, Vec4);

impl Formatter for f32 {
    fn debug(&self) -> String {
        let p = Args::get_precision() as u32;
        let s = fmt_float(self, p * 2 + 1, p);
        if s.find(".") != None && s.ends_with("0") {
            log!(" *** {} -> {} *** ", self, s);
        }
        return s;
    }
}

//#endregion
//#region float precision

static G_PRECISION: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(4));
impl Args {
    pub fn set_precision(precision: u8) {
        let mut v = G_PRECISION.lock().unwrap();
        *v = precision;
    }
    pub fn get_precision() -> u8 {
        let v = G_PRECISION.lock().unwrap();
        return *v;
    }
}

//#endregion
