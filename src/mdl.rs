use crate::*;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

macro_rules! MdlReadType1 {
    ($block:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $block.typ == stringify!($ty) {
            $var = <$ty>::read_mdl(&$block).or_else(|e| ERR!("{}: {}", TNAME!($ty), e))?;
            return Ok(());
        })+
    };
}
macro_rules! MdlReadType2 {
    ($block:expr, $( $ty:ty => $name:expr => $var:expr ),+ $(,)?) => {
        $(if $block.typ == F!("{}s", stringify!($ty)) {
            for a in &$block.blocks {
                if a.typ == $name {
                    $var.push(<$ty>::read_mdl(a).or_else(|e| ERR!("{}: {}", TNAME!($ty), e))?);
                }
            }
            return Ok(());
        })+
    };
}
macro_rules! MdlReadType3 {
    ($block:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $block.typ == stringify!($ty) {
            $var.push(<$ty>::read_mdl(&$block).or_else(|e| ERR!("{}: {}", TNAME!($ty), e))?);
            return Ok(());
        })+
    };
}
macro_rules! MdlReadType4 {
    ($block:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $block.typ == F!("{}s", stringify!($ty)) {
            for a in &$block.fields {
                $var.push(<$ty>::read_mdl(a).or_else(|e| ERR!("{}: {}", TNAME!($ty), e))?);
            }
            return Ok(());
        })+
    };
}

#[macro_export]
macro_rules! MdlWriteType1 {
    ($lines:ident, $depth:expr, $( $var:expr ),+ $(,)?) => {
        $( $lines.append(&mut $var.write_mdl($depth)?); )+
    };
}
#[macro_export]
macro_rules! MdlWriteType2 {
    ($lines:ident, $depth:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            let indent = indent!($depth);
            $lines.push(F!("{indent}{} {} {{", $name, $var.len()));
            for a in &$var {
                MdlWriteType1!($lines, $depth+1, a);
            }
            $lines.push(F!("{indent}}}"));
        })+
    };
}
#[macro_export]
macro_rules! MdlWriteType3 {
    ($lines:ident, $depth:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            for a in &$var {
                $lines.push(F!("{} {{", $name));
                MdlWriteType1!($lines, $depth+1, a);
                $lines.push(F!("}}"));
            }
        })+
    };
}
#[macro_export] // Nodes
macro_rules! MdlWriteType4 {
    ($lines:ident, $depth:expr, $member:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            for a in &$var {
                paste!{ $lines.push(F!("{} \"{}\" {{", $name, a.$member)); }
                MdlWriteType1!($lines, $depth+1, a);
                $lines.push(F!("}}"));
            }
        })+
    };
}

#[derive(Parser)]
#[grammar = "mdl.pest"]
pub struct MdlParser;

//#region structs

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

#[derive(Debug, Default)]
pub enum MdlValue {
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
impl MdlValue {
    pub fn from(p: Pair<'_, Rule>) -> Result<Self, MyError> {
        match p.as_rule() {
            Rule::integer => Ok(Self::Integer(p.as_str().parse().unwrap())),
            Rule::float => Ok(Self::Float(p.as_str().parse().unwrap())),
            Rule::identifier => Ok(Self::Flag(p.as_str().parse().unwrap())),
            Rule::string => Ok(Self::String(Self::unwrap_string(p.as_str()))),
            Rule::identifier_array => {
                Ok(Self::FlagArray(p.into_inner().into_iter().map(|p| p.as_str().parse().unwrap()).collect()))
            },
            Rule::number_array => {
                let inner = p.into_inner();
                let mut fv = Vec::<f32>::with_capacity(inner.len());
                let mut iv = Vec::<i32>::with_capacity(fv.capacity());
                for p in inner {
                    let s = p.as_str();
                    if p.as_rule() == Rule::float {
                        fv.push(s.parse().unwrap());
                    } else {
                        // 19: number of digits in i64:MAX
                        let i = if s.len() < 19 {
                            s.parse::<i64>().unwrap()
                        } else {
                            yesno!(s.starts_with('-'), i64::MIN, i64::MAX)
                        };
                        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                            iv.push(i as i32);
                        }
                        fv.push(s.parse().unwrap());
                    }
                }
                yesno!(iv.len() == fv.len(), Ok(Self::IntegerArray(iv)), Ok(Self::FloatArray(fv)))
            },
            _impossible => Ok(Self::None),
        }
    }

    pub fn to<T: FromMdlValue>(&self) -> T {
        T::from(&self)
    }

    pub fn unwrap_string(s: &str) -> String {
        let s = &s[1..s.len() - 1];
        s.to_string()
    }

    pub fn as_str(&self) -> &str {
        match self {
            MdlValue::String(s) => s,
            MdlValue::Flag(s) => s,
            _ => "",
        }
    }
}

//#endregion

impl MdlxData {
    pub fn write_mdl(&mut self, path: &Path) -> Result<(), MyError> {
        let mut lines: Vec<String> = vec![];
        Self::write_mdl_comment(&mut lines, path);

        MdlWriteType1!(lines, 0, self.version, self.model);
        MdlWriteType2!(lines, 0,
            "Sequences"         => self.sequences,
            "GlobalSequences"   => self.globalseqs,
            "Textures"          => self.textures,
            "Materials"         => self.materials,
            "TextureAnims"      => self.texanims,
            "PivotPoints"       => self.pivot_points,
        );
        MdlWriteType3!(lines, 0,
            "Geoset"            => self.geosets,
            "GeosetAnim"        => self.geoanims,
        );
        MdlWriteType4!(lines, 0, base.name,
            "Bone"              => self.bones,
            "Light"             => self.lights,
            "Helper"            => self.helpers,
            "Attachment"        => self.attachments,
            "ParticleEmitter"   => self.particle_emitters,
            "ParticleEmitter2"  => self.particle_emitters2,
            "RibbonEmitter"     => self.ribbon_emitters,
            "EventObject"       => self.eventobjs,
            "CollisionShape"    => self.collisions,
        );
        MdlWriteType4!(lines, 0, name, "Camera" => self.cameras );

        let line_ending = line_ending!();
        let text = lines.join(line_ending) + line_ending;
        return Ok(std::fs::write(path, text)?);
    }
    fn write_mdl_comment(lines: &mut Vec<String>, path: &Path) {
        let width = 48;
        let comments = vec![
            F!("{:?}", path.file_name().unwrap()),
            F!("Generated by {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S (%Z)").to_string(),
            F!("{}", env!("CARGO_PKG_HOMEPAGE")),
        ];
        lines.push(F!("//+{}+", "-".repeat(width)));
        for c in comments {
            lines.push(F!("//|{:^w$}|", c, w = width));
        }
        lines.push(F!("//+{}+", "-".repeat(width)));
    }

    pub fn read_mdl(input: &str) -> Result<Self, MyError> {
        let mdl = MdlParser::parse(Rule::file, input).map_err(|e| F!("Failed to parse mdl: {}", e))?;
        let mut this = MdlxData::default();

        for pair in mdl {
            if let Rule::file = pair.as_rule() {
                for p in pair.into_inner() {
                    if let Rule::block = p.as_rule() {
                        this.parse_mdl_block(MdlBlock::from(p)?)?;
                    }
                }
                break; // only 1 [file] rule
            }
        }

        return Ok(this);
    }

    fn parse_mdl_block(&mut self, block: MdlBlock) -> Result<(), MyError> {
        MdlReadType1!(block,
            Version     => self.version,
            Model       => self.model,
        );
        MdlReadType2!(block,
            Sequence    => "Anim"       => self.sequences,
            Texture     => "Bitmap"     => self.textures,
            TextureAnim => "TVertexAnim"=> self.texanims,
            Material    => "Material"   => self.materials,
        );
        MdlReadType3!(block,
            Geoset          => self.geosets,
            GeosetAnim      => self.geoanims,
            Bone            => self.bones,
            Light           => self.lights,
            Helper          => self.helpers,
            Attachment      => self.attachments,
            ParticleEmitter => self.particle_emitters,
            ParticleEmitter2=> self.particle_emitters2,
            RibbonEmitter   => self.ribbon_emitters,
            EventObject     => self.eventobjs,
            CollisionShape  => self.collisions,
            Camera          => self.cameras,
        );
        MdlReadType4!(block,
            GlobalSequence  => self.globalseqs,
            PivotPoint      => self.pivot_points,
        );
        return Ok(());
    }
}

//#region trait: FromMdlValue

pub trait FromMdlValue {
    fn from(v: &MdlValue) -> Self;
}

impl FromMdlValue for i32 {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::Integer(i) = v { *i } else { Self::default() }
    }
}
impl FromMdlValue for u32 {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::Integer(iv) = v { yesno!(*iv < 0, 0u32, *iv as u32) } else { Self::default() }
    }
}
impl FromMdlValue for Vec<i32> {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::IntegerArray(iv) = v { iv.clone() } else { Self::default() }
    }
}
impl FromMdlValue for Vec<u32> {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::IntegerArray(iv) = v {
            iv.convert(|v| yesno!(*v < 0, 0u32, *v as u32))
        } else {
            Self::default()
        }
    }
}

impl FromMdlValue for i8 {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::Integer(i) = v { *i as i8 } else { Self::default() }
    }
}
impl FromMdlValue for u8 {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::Integer(iv) = v { yesno!(*iv < 0, 0u8, *iv as u8) } else { Self::default() }
    }
}
impl FromMdlValue for Vec<i8> {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::IntegerArray(iv) = v { iv.convert(|v| *v as i8) } else { Self::default() }
    }
}
impl FromMdlValue for Vec<u8> {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::IntegerArray(iv) = v { iv.convert(|v| yesno!(*v < 0, 0u8, *v as u8)) } else { Self::default() }
    }
}

impl FromMdlValue for f32 {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::Float(f) => *f,
            MdlValue::Integer(i) => *i as f32,
            _ => Self::default(),
        }
    }
}
impl FromMdlValue for Vec<f32> {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::FloatArray(fv) => fv.clone(),
            MdlValue::IntegerArray(iv) => iv.convert(|v| *v as f32),
            _ => Self::default(),
        }
    }
}
impl FromMdlValue for Vec2 {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::FloatArray(fv) => Self::from_slice(fv.as_slice()),
            MdlValue::IntegerArray(iv) => Self::from_slice(iv.convert(|v| *v as f32).as_slice()),
            _ => Self::default(),
        }
    }
}
impl FromMdlValue for Vec3 {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::FloatArray(fv) => Self::from_slice(fv.as_slice()),
            MdlValue::IntegerArray(iv) => Self::from_slice(iv.convert(|v| *v as f32).as_slice()),
            _ => Self::default(),
        }
    }
}
impl FromMdlValue for Vec4 {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::FloatArray(fv) => Self::from_slice(fv.as_slice()),
            MdlValue::IntegerArray(iv) => Self::from_slice(iv.convert(|v| *v as f32).as_slice()),
            _ => Self::default(),
        }
    }
}

impl FromMdlValue for String {
    fn from(v: &MdlValue) -> Self {
        match v {
            MdlValue::String(s) => s.clone(),
            MdlValue::Flag(s) => s.clone(),
            _ => String::default(),
        }
    }
}
impl FromMdlValue for Vec<String> {
    fn from(v: &MdlValue) -> Self {
        if let MdlValue::FlagArray(sv) = v { sv.clone() } else { Self::default() }
    }
}

pub trait _ExtendMdlFieldArray {
    fn to<T: FromMdlFieldArray>(&self) -> T;
}
impl _ExtendMdlFieldArray for Vec<MdlField> {
    fn to<T: FromMdlFieldArray>(&self) -> T {
        T::from(&self)
    }
}

//#endregion
//#region trait: FromMdlFieldArray

pub trait FromMdlFieldArray {
    fn from(v: &Vec<MdlField>) -> Self;
}

impl FromMdlFieldArray for Vec<i32> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| FromMdlValue::from(&f.value))
    }
}
impl FromMdlFieldArray for Vec<u32> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| FromMdlValue::from(&f.value))
    }
}

impl FromMdlFieldArray for Vec<i8> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| {
            let i: i32 = FromMdlValue::from(&f.value);
            i as i8
        })
    }
}
impl FromMdlFieldArray for Vec<u8> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| {
            let i: i32 = FromMdlValue::from(&f.value);
            i as u8
        })
    }
}

impl FromMdlFieldArray for Vec<i16> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| {
            let i: i32 = FromMdlValue::from(&f.value);
            i as i16
        })
    }
}
impl FromMdlFieldArray for Vec<u16> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| {
            let i: i32 = FromMdlValue::from(&f.value);
            i as u16
        })
    }
}

impl FromMdlFieldArray for Vec<Vec2> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| FromMdlValue::from(&f.value))
    }
}
impl FromMdlFieldArray for Vec<Vec3> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| FromMdlValue::from(&f.value))
    }
}
impl FromMdlFieldArray for Vec<Vec4> {
    fn from(v: &Vec<MdlField>) -> Self {
        v.convert(|f| FromMdlValue::from(&f.value))
    }
}

//#endregion
