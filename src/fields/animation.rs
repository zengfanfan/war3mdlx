use crate::*;

pub trait TAnimation: ReadFromCursor + WriteToCursor + FromMdlValue + std::fmt::Debug + Default + Formatter {}
impl<T> TAnimation for T where T: ReadFromCursor + WriteToCursor + FromMdlValue + std::fmt::Debug + Default + Formatter {}

#[derive(Dbg, Default)]
pub struct KeyFrame<T: TAnimation> {
    pub frame: i32,
    pub value: T,
    pub itan: T,
    pub otan: T,
    #[dbg(skip)]
    pub has_tans: bool,
}

#[derive(Dbg, Default)]
pub struct Animation<T: TAnimation> {
    pub interp_type: InterpolationType,
    pub global_seq_id: i32,
    #[dbg(formatter = "fmtx")]
    pub key_frames: Vec<KeyFrame<T>>,
}

impl<T: TAnimation> Animation<T> {
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        let kfn = cur.readx()?;
        this.interp_type = InterpolationType::from(cur.readx()?);
        this.global_seq_id = cur.readx()?;

        if let InterpolationType::Error(v) = this.interp_type {
            return ERR!("Unknown interpolation type: {}", v);
        }

        this.key_frames = Vec::with_capacity(kfn as usize);
        for _ in 0..kfn {
            let has_tans = this.interp_type.has_tans();
            this.key_frames.push(KeyFrame {
                frame: cur.readx()?,
                value: cur.readx()?,
                itan: cur.read_if(has_tans, T::default())?,
                otan: cur.read_if(has_tans, T::default())?,
                has_tans,
            });
        }

        return Ok(this);
    }

    pub fn write_mdx(&self, chunk: &mut MdxChunk, id: &u32) -> Result<(), MyError> {
        chunk.write_be(id)?;
        chunk.write(&self.key_frames.len())?;
        chunk.write(&self.interp_type.to())?;
        chunk.write(&self.global_seq_id)?;
        for kf in &self.key_frames {
            chunk.write(&kf.frame)?;
            chunk.write(&kf.value)?;
            if kf.has_tans {
                chunk.write(&kf.itan)?;
                chunk.write(&kf.otan)?;
            }
        }
        return Ok(());
    }
    pub fn calc_mdx_size(&self) -> u32 {
        let mut sz = 16; // id + kfn + interp_type + global_seq_id
        for kf in &self.key_frames {
            sz += 4 + T::size(); // frame + value
            if kf.has_tans {
                sz += T::size() * 2; // intan + outan
            }
        }
        return sz;
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Build! { global_seq_id: -1 };
        for f in &block.fields {
            if let MdlValue::None = f.value {
                this.interp_type = InterpolationType::from_str(f.name.as_str(), this.interp_type);
            } else if f.name == "GlobalSeqId" {
                this.global_seq_id = f.value.to();
            }
        }
        for f in &block.frames {
            this.key_frames.push(KeyFrame {
                frame: f.frame,
                value: f.value.to(),
                itan: f.intan.to(),
                otan: f.outan.to(),
                has_tans: this.interp_type.has_tans(),
            });
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}{:?},", self.interp_type));
        lines.push_if_nneg1(&F!("{indent}GlobalSeqId"), &self.global_seq_id);
        for kf in &self.key_frames {
            lines.pushx(&F!("{indent}{}:", kf.frame), &kf.value);
            if kf.has_tans {
                lines.pushx(&F!("{indent2}InTan"), &kf.itan);
                lines.pushx(&F!("{indent2}OutTan"), &kf.otan);
            }
        }
        return Ok(lines);
    }

    pub fn convert<F: Fn(&T) -> T>(&self, f: F) -> Self {
        let mut this = Build! { interp_type: self.interp_type, global_seq_id: self.global_seq_id };
        for kf in &self.key_frames {
            this.key_frames.push(KeyFrame {
                frame: kf.frame,
                value: f(&kf.value),
                itan: f(&kf.itan),
                otan: f(&kf.otan),
                has_tans: kf.has_tans,
            });
        }
        return this;
    }
}

#[macro_export]
macro_rules! MdlWriteAnim {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr ),+ $(,)?) => {
        $(
            let indent = indent!(&$depth);
            $lines.push(F!("{}{} {} {{", indent, $name, $avar.key_frames.len()));
            $lines.append($avar.write_mdl($depth + 1)?.as_mut());
            $lines.push(F!("{}}}", indent));
        )+
    };
}
#[macro_export]
macro_rules! MdlWriteAnimIfSome {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr ),+ $(,)?) => {
        $(if let Some(item) = &$avar {
            MdlWriteAnim!($lines, $depth, $name => item);
        })+
    };
}

#[macro_export]
macro_rules! MdlWriteAnimStatic {
    ($lines:ident, $depth:expr, $( $name:expr => $svar:expr ),+ $(,)?) => {
        $(
            let indent = indent!($depth);
            $lines.push(F!("{}static {} {},", indent, $name, fmtx(&$svar)));
        )+
    };
}
#[macro_export]
macro_rules! MdlWriteAnimStaticIfNone {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr => $def:expr => $svar:expr ),+ $(,)?) => {
        $(if let None = &$avar && $svar != $def {
            MdlWriteAnimStatic!($lines, $depth, $name => $svar);
        })+
    };
}
#[macro_export]
macro_rules! MdlWriteAnimBoth {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr => $def:expr => $svar:expr ),+ $(,)?) => {
        $(if let Some(item) = &$avar {
            MdlWriteAnim!($lines, $depth, $name => item);
        } else if $svar != $def {
            MdlWriteAnimStatic!($lines, $depth, $name => $svar);
        })+
    };
}

#[macro_export]
macro_rules! MdxWriteAnim {
    ($chunk:ident, $( $id:expr => $avar:expr ),+ $(,)?) => {
        $(
            $avar.write_mdx($chunk, &$id)?;
        )+
    };
}

//#region InterpolationType

#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InterpolationType {
    #[default]
    DontInterp,
    Linear,
    Hermite,
    Bezier,
    Error(i32),
}

impl InterpolationType {
    fn from(v: i32) -> Self {
        match v {
            0 => Self::DontInterp,
            1 => Self::Linear,
            2 => Self::Hermite,
            3 => Self::Bezier,
            x => Self::Error(x),
        }
    }

    fn from_str(s: &str, def: Self) -> Self {
        match_istr!(s,
            "DontInterp" => Self::DontInterp,
            "Linear" => Self::Linear,
            "Hermite" => Self::Hermite,
            "Bezier" => Self::Bezier,
            _err => def,
        )
    }

    fn to(&self) -> i32 {
        match self {
            Self::DontInterp => 0,
            Self::Linear => 1,
            Self::Hermite => 2,
            Self::Bezier => 3,
            Self::Error(x) => *x,
        }
    }

    fn has_tans(&self) -> bool {
        matches!(self, Self::Hermite | Self::Bezier)
    }
}

//#endregion
//#region formatter

impl<T: TAnimation> Formatter for KeyFrame<T> {
    fn fmt(&self) -> String {
        match self.has_tans {
            true => {
                F!("{}: {}, InTan={}, OutTan={},", self.frame, fmtx(&self.value), fmtx(&self.itan), fmtx(&self.otan))
            },
            false => F!("{}: {},", self.frame, fmtx(&self.value)),
        }
    }
}
impl<T: TAnimation> Formatter for Vec<KeyFrame<T>> {
    fn fmt(&self) -> String {
        let mut list: Vec<String> = Vec::with_capacity(self.len());
        for kf in self {
            list.push(fmtx(kf));
        }
        return F!("[\n    {}\n]", list.join("\n    "));
    }
}

//#endregion
//#region _ExtendSomeAnimation

pub trait _ExtendSomeAnimation {
    fn calc_mdx_size(&self) -> u32;
    fn write_mdx(&self, chunk: &mut MdxChunk, id: &u32) -> Result<(), MyError>;
}

impl<T: TAnimation> _ExtendSomeAnimation for Option<Animation<T>> {
    fn calc_mdx_size(&self) -> u32 {
        match self {
            Some(a) => a.calc_mdx_size(),
            None => 0,
        }
    }

    fn write_mdx(&self, chunk: &mut MdxChunk, id: &u32) -> Result<(), MyError> {
        match self {
            Some(a) => a.write_mdx(chunk, id),
            None => Ok(()),
        }
    }
}

//#endregion
