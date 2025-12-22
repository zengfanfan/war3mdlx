use crate::*;

//#region MdlBlock

#[derive(Dbg, Default)]
pub struct MdlBlock {
    pub typ: String,
    pub name: String,
    #[dbg(formatter = "fmtx")]
    pub fields: Vec<MdlField>,
    pub frames: Vec<MdlFrame>,
    pub blocks: Vec<MdlBlock>,
}

impl MdlBlock {
    pub fn from(pair: Pair<'_, Rule>) -> Result<Self, MyError> {
        let mut this = Build!();
        let inner = pair.into_inner();
        for p in inner {
            match p.as_rule() {
                Rule::identifier => this.typ = p.as_str().to_string(),
                Rule::string => this.name = MdlValue::unwrap_string(p.as_str()),
                Rule::block => this.blocks.push(MdlBlock::from(p)?),
                Rule::field => this.fields.push(MdlField::from(p)?),
                Rule::frame => this.frames.push(MdlFrame::from(p)?),
                _other => (), // ignore integers
            }
        }
        return Ok(this);
    }
}

//#endregion
//#region MdlField

#[derive(Dbg, Default)]
pub struct MdlField {
    pub name: String,
    #[dbg(fmt = "{:?}")]
    pub value: MdlValue, // option
}

impl MdlField {
    pub fn from(pair: Pair<'_, Rule>) -> Result<Self, MyError> {
        let mut this = Build!();
        let inner = pair.into_inner();
        let mut first_ident = true;
        for p in inner {
            match p.as_rule() {
                Rule::identifier => {
                    if first_ident {
                        this.name = p.as_str().to_string();
                        first_ident = false;
                    } else {
                        this.value = MdlValue::from(p)?;
                    }
                },
                _value => this.value = MdlValue::from(p)?,
            }
        }
        return Ok(this);
    }
}

impl Formatter for MdlField {
    fn fmt(&self) -> String {
        F!("{} = {:?}", self.name, &self.value)
    }
}

impl Formatter for Vec<MdlField> {
    fn fmt(&self) -> String {
        F!("{:#?}", self.iter().map(|x| Formatter::fmt(x)).collect::<Vec<_>>())
    }
}

//#endregion
//#region MdlFrame

#[derive(Dbg, Default)]
pub struct MdlFrame {
    pub frame: i32,
    #[dbg(fmt = "{:?}")]
    pub value: MdlValue,
    #[dbg(fmt = "{:?}")]
    pub intan: MdlValue,
    #[dbg(fmt = "{:?}")]
    pub outan: MdlValue,
}

impl MdlFrame {
    pub fn from(pair: Pair<'_, Rule>) -> Result<Self, MyError> {
        let mut this = Build!();
        let mut inner = pair.into_inner();
        this.frame = inner.next().unwrap().as_str().parse().unwrap();
        this.value = MdlValue::from(inner.next().unwrap())?;
        if !inner.is_empty() {
            this.intan = MdlValue::from(inner.next().unwrap())?;
            this.outan = MdlValue::from(inner.next().unwrap())?;
        }
        return Ok(this);
    }
}

//#endregion
//#region MdlValue

#[derive(Debug, Default, PartialEq)]
pub enum MdlValueType {
    #[default]
    None,
    Integer(i32),
    Float(f32),
    String(String),
    Flag(String),
    IntegerArray(Vec<i32>),
    FloatArray(Vec<f32>),
    FlagArray(Vec<String>),
}

#[derive(Debug, Default)]
pub struct MdlValue {
    pub typ: MdlValueType,
    pub line: u32,
}

impl MdlValue {
    pub fn from(p: Pair<'_, Rule>) -> Result<Self, MyError> {
        let line = p.as_span().start_pos().line_col().0 as u32;
        Ok(match p.as_rule() {
            Rule::integer => Self::Integer(line, p.as_str().parse()?),
            Rule::float => Self::Float(line, p.as_str().parse()?),
            Rule::identifier => Self::Flag(line, p.as_str().s()),
            Rule::string => Self::String(line, Self::unwrap_string(p.as_str())),
            Rule::identifier_array => {
                Self::FlagArray(line, p.into_inner().into_iter().map(|p| p.as_str().s()).collect())
            },
            Rule::number_array => {
                let inner = p.into_inner();
                let mut fv = Vec::<f32>::with_capacity(inner.len());
                let mut iv = Vec::<i32>::with_capacity(fv.capacity());
                for p in inner {
                    let s = p.as_str();
                    if p.as_rule() == Rule::float {
                        fv.push(s.parse()?);
                    } else {
                        // 19: number of digits in i64:MAX
                        let i = if s.len() < 19 {
                            s.parse::<i64>()?
                        } else {
                            yesno!(s.starts_with('-'), i64::MIN, i64::MAX)
                        };
                        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                            iv.push(i as i32);
                        }
                        fv.push(s.parse()?);
                    }
                }
                yesno!(iv.len() == fv.len(), Self::IntegerArray(line, iv), Self::FloatArray(line, fv))
            },
            _impossible => Self::default(),
        })
    }

    pub fn to<T: FromMdlValue>(&self) -> Result<T, MyError> {
        T::from(&self)
    }

    pub fn unwrap_string(s: &str) -> String {
        let s = &s[1..s.len() - 1]; // remove quotes
        s.unescape()
    }

    pub fn as_str(&self) -> &str {
        match &self.typ {
            MdlValueType::String(s) => s,
            MdlValueType::Flag(s) => s,
            _ => "",
        }
    }

    pub fn expect<T>(&self, s: &str) -> Result<T, MyError> {
        ERR!("Expected {} at line {}", s, self.line)
    }
}

#[allow(non_snake_case)]
impl MdlValue {
    fn Integer(line: u32, v: i32) -> Self {
        Self { line, typ: MdlValueType::Integer(v) }
    }
    fn Float(line: u32, v: f32) -> Self {
        Self { line, typ: MdlValueType::Float(v) }
    }
    fn String(line: u32, v: String) -> Self {
        Self { line, typ: MdlValueType::String(v) }
    }
    fn Flag(line: u32, v: String) -> Self {
        Self { line, typ: MdlValueType::Flag(v) }
    }
    fn IntegerArray(line: u32, v: Vec<i32>) -> Self {
        Self { line, typ: MdlValueType::IntegerArray(v) }
    }
    fn FloatArray(line: u32, v: Vec<f32>) -> Self {
        Self { line, typ: MdlValueType::FloatArray(v) }
    }
    fn FlagArray(line: u32, v: Vec<String>) -> Self {
        Self { line, typ: MdlValueType::FlagArray(v) }
    }
}

//#endregion
//#region trait: FromMdlValue

pub trait FromMdlValue {
    fn from(v: &MdlValue) -> Result<Self, MyError>
    where
        Self: Sized;
}

macro_rules! impl_FromMdlValue_int {
    ($($ty:ty),*) => {
        $(
            impl FromMdlValue for $ty {
                fn from(v: &MdlValue) -> Result<Self, MyError> {
                    if let MdlValueType::Integer(i) = &v.typ { Ok(*i as $ty) } else { v.expect("integer") }
                }
            }
            impl FromMdlValue for Vec<$ty> {
                fn from(v: &MdlValue) -> Result<Self, MyError> {
                    if let MdlValueType::IntegerArray(iv) = &v.typ {
                        Ok(iv.convert(|v| *v as $ty))
                    } else {
                        v.expect("integer array")
                    }
                }
            }
        )*
    };
}
macro_rules! impl_FromMdlValue_uint {
    ($($ty:ty),*) => {
        $(
            impl FromMdlValue for $ty {
                fn from(v: &MdlValue) -> Result<Self, MyError> {
                    if let MdlValueType::Integer(i) = &v.typ { Ok(yesno!(*i < 0, 0, *i as $ty)) } else { v.expect("integer") }
                }
            }
            impl FromMdlValue for Vec<$ty> {
                fn from(v: &MdlValue) -> Result<Self, MyError> {
                    if let MdlValueType::IntegerArray(iv) = &v.typ {
                        Ok(iv.convert(|v| yesno!(*v < 0, 0, *v as $ty)))
                    } else {
                        v.expect("integer array")
                    }
                }
            }
        )*
    };
}
macro_rules! impl_FromMdlValue_vec234 {
    ($($a:tt),*) => {
        $(paste! {
            impl FromMdlValue for [<Vec $a>] {
                fn from(v: &MdlValue) -> Result<Self, MyError> {
                    const LEN: usize = $a;
                    let vs = match &v.typ {
                        MdlValueType::FloatArray(fv) => fv.to_vec(),
                        MdlValueType::IntegerArray(iv) => iv.convert(|v| *v as f32),
                        _ => vec![],
                    };
                    if vs.len() == LEN { Ok(Self::from_slice(vs.as_slice())) } else { v.expect(&F!("{} numbers", LEN)) }
                }
            }
        })*
    };
}

impl_FromMdlValue_int!(i8, i16, i32);
impl_FromMdlValue_uint!(u8, u16, u32);
impl_FromMdlValue_vec234!(2, 3, 4);

impl FromMdlValue for f32 {
    fn from(v: &MdlValue) -> Result<Self, MyError> {
        match &v.typ {
            MdlValueType::Float(f) => Ok(*f),
            MdlValueType::Integer(i) => Ok(*i as f32),
            _ => v.expect("number"),
        }
    }
}
impl FromMdlValue for Vec<f32> {
    fn from(v: &MdlValue) -> Result<Self, MyError> {
        match &v.typ {
            MdlValueType::FloatArray(fv) => Ok(fv.clone()),
            MdlValueType::IntegerArray(iv) => Ok(iv.convert(|v| *v as f32)),
            _ => v.expect("number array"),
        }
    }
}

impl FromMdlValue for String {
    fn from(v: &MdlValue) -> Result<Self, MyError> {
        match &v.typ {
            MdlValueType::String(s) => Ok(s.clone()),
            MdlValueType::Flag(s) => Ok(s.clone()),
            _ => v.expect("string"),
        }
    }
}
impl FromMdlValue for Vec<String> {
    fn from(v: &MdlValue) -> Result<Self, MyError> {
        if let MdlValueType::FlagArray(sv) = &v.typ { Ok(sv.clone()) } else { v.expect("string array") }
    }
}

pub trait _ExtendMdlFieldArray {
    fn to<T: FromMdlFieldArray>(&self) -> Result<T, MyError>;
}
impl _ExtendMdlFieldArray for Vec<MdlField> {
    fn to<T: FromMdlFieldArray>(&self) -> Result<T, MyError> {
        T::from(&self)
    }
}

//#endregion
//#region trait: FromMdlFieldArray

pub trait FromMdlFieldArray {
    fn from(v: &Vec<MdlField>) -> Result<Self, MyError>
    where
        Self: Sized;
}

macro_rules! impl_FromMdlFieldArray {
    ($($ty:ty),*) => {
        $(impl FromMdlFieldArray for Vec<$ty> {
            fn from(v: &Vec<MdlField>) -> Result<Self, MyError> {
                v.try_convert(|f| FromMdlValue::from(&f.value))
            }
        })*
    };
}

impl_FromMdlFieldArray!(i8, u8, i16, u16, i32, u32, f32, Vec2, Vec3, Vec4);

//#endregion
