use crate::*;

//#region trait: _ExtendPair

trait _ExtendPair {
    fn lineno(&self) -> u32;
}
impl _ExtendPair for Pair<'_, Rule> {
    fn lineno(&self) -> u32 {
        self.as_span().start_pos().line_col().0 as u32
    }
}

//#endregion
//#region MdlBlock

#[derive(Dbg, Default)]
pub struct MdlBlock {
    pub typ: String,
    pub name: String,
    pub line: u32,
    #[dbg(formatter = "fmtx")]
    pub fields: Vec<MdlField>,
    pub frames: Vec<MdlFrame>,
    pub blocks: Vec<MdlBlock>,
}

impl MdlBlock {
    pub fn from(pair: Pair<'_, Rule>) -> Result<Self, MyError> {
        let mut this = Build! {line: pair.lineno()};
        let inner = pair.into_inner();
        for p in inner {
            match p.as_rule() {
                Rule::identifier => this.typ = p.as_str().to_string(),
                Rule::string => this.name = MdlValue::unwrap_string(p.as_str()),
                Rule::block => this.blocks.push(MdlBlock::from(p)?),
                Rule::field => this.fields.push(MdlField::from(p, &this.typ)?),
                Rule::frame => this.frames.push(MdlFrame::from(p, &this.typ)?),
                _other => (), // ignore integers
            }
        }
        return Ok(this);
    }

    pub fn unexpect<T>(&self) -> Result<T, MyError> {
        ERR!("Unexpected {}({:?}) at line {}.", self.typ, self.name, self.line)
    }
}

//#endregion
//#region MdlField

#[derive(Dbg, Default)]
pub struct MdlField {
    pub name: String,
    pub scope: String,
    pub line: u32,
    #[dbg(fmt = "{:?}")]
    pub value: MdlValue, // option
}

impl MdlField {
    pub fn from(pair: Pair<'_, Rule>, scope: &str) -> Result<Self, MyError> {
        let mut this = Build! {scope: scope.s(), line: pair.lineno()};
        this.value.line = this.line;
        let inner = pair.into_inner();
        let mut first_ident = true;
        for p in inner {
            if let Rule::identifier = p.as_rule() {
                if first_ident {
                    this.name = p.as_str().to_string();
                    first_ident = false;
                } else {
                    this.value = MdlValue::from(p, &this.name)?;
                }
            } else {
                this.value = MdlValue::from(p, &this.name)?;
            }
        }
        return Ok(this);
    }

    pub fn unexpect<T>(&self) -> Result<T, MyError> {
        let name = yesno!(self.name.is_empty(), &self.value.raw, &self.name);
        ERR!("Unexpected {:?} (in {}) at line {}.", name, self.scope, self.line)
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
    pub scope: String,
    pub line: u32,
    pub frame: i32,
    #[dbg(fmt = "{:?}")]
    pub value: MdlValue,
    #[dbg(fmt = "{:?}")]
    pub intan: MdlValue,
    #[dbg(fmt = "{:?}")]
    pub outan: MdlValue,
}

impl MdlFrame {
    pub fn from(pair: Pair<'_, Rule>, scope: &str) -> Result<Self, MyError> {
        let mut this = Build! {scope: scope.s(), line: pair.lineno()};
        let mut inner = pair.into_inner();
        this.frame = inner.next().unwrap().as_str().parse().unwrap();
        this.value = MdlValue::from(inner.next().unwrap(), &this.scope)?;
        for p in inner {
            let f = MdlField::from(p, &this.scope)?;
            match_istr!(f.name.as_str(),
                "Intan" => this.intan = f.value,
                "OutTan" => this.outan = f.value,
                _other => EXIT1!("Unexpected {:?} (in {}) at line {}",
                f.name.or(&f.value.raw), this.scope, f.value.line),
            );
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
    pub name: String,
    pub raw: String,
    pub typ: MdlValueType,
    pub line: u32,
}

impl Display for MdlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl MdlValue {
    pub fn from(p: Pair<'_, Rule>, name: &str) -> Result<Self, MyError> {
        let raw = p.as_str();
        let mut this = Build! {name: name.s(), line: p.lineno(), raw: raw.s()};
        this.typ = match p.as_rule() {
            Rule::integer => MdlValueType::Integer(raw.parse()?),
            Rule::float => MdlValueType::Float(raw.parse()?),
            Rule::identifier => MdlValueType::Flag(raw.s()),
            Rule::string => MdlValueType::String(Self::unwrap_string(raw)),
            Rule::identifier_array => {
                MdlValueType::FlagArray(p.into_inner().into_iter().map(|p| p.as_str().s()).collect())
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
                yesno!(iv.len() == fv.len(), MdlValueType::IntegerArray(iv), MdlValueType::FloatArray(fv))
            },
            _impossible => MdlValueType::default(),
        };
        return Ok(this);
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
        let forname = yesno!(self.name.is_empty(), "".s(), F!(" for {:?}", self.name));
        let gottype = yesno!(self.typ == MdlValueType::None, "".s(), F!(", got {:?}", self.typ));
        ERR!("Expected {}{} at line {}{}.", s, forname, self.line, gottype)
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
                    match &v.typ {
                        MdlValueType::Integer(i) => Ok(*i as $ty),
                        MdlValueType::Flag(f) if f.eq_icase("None") || f.eq_icase("Multiple") => Ok(-1),
                        _ => v.expect("integer"),
                    }
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
