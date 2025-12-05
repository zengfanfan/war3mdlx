use crate::*;

pub trait TAnimation: ReadFromCursor + std::fmt::Debug + Default + Formatter {}
impl<T> TAnimation for T where T: ReadFromCursor + std::fmt::Debug + Default + Formatter {}

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
    #[dbg(formatter = "fmt_id4s")]
    pub id: u32,
    pub interp_type: InterpolationType,
    pub global_seq_id: i32,
    #[dbg(formatter = "fmt_key_frames")]
    pub key_frames: Vec<KeyFrame<T>>,
}

impl<T: TAnimation> Animation<T> {
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>, id: u32) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.id = id;
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

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}{:?},", self.interp_type));
        lines.push_if_nneg1(&F!("{indent}GlobalSeqId"), &self.global_seq_id);
        for kf in &self.key_frames {
            lines.push(F!("{indent}{}: {},", kf.frame, fmtx(&kf.value)));
            if kf.has_tans {
                lines.push(F!("{indent2}InTan {},", fmtx(&kf.itan)));
                lines.push(F!("{indent2}OutTan {},", fmtx(&kf.otan)));
            }
        }
        return Ok(lines);
    }

    pub fn convert<F: Fn(&T) -> T>(&self, f: F) -> Self {
        let mut this = Self::default();
        this.id = self.id;
        this.interp_type = self.interp_type;
        this.global_seq_id = self.global_seq_id;
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
macro_rules! _MdlWriteAnim {
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
macro_rules! MdlWriteAnim {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr ),+ $(,)?) => {
        $(if let Some(item) = &$avar {
            _MdlWriteAnim!($lines, $depth, $name => item);
        })+
    };
}

#[macro_export]
macro_rules! _MdlWriteAnimStatic {
    ($lines:ident, $depth:expr, $( $name:expr => $svar:expr ),+ $(,)?) => {
        $(
            let indent = indent!($depth);
            $lines.push(F!("{}static {} {},", indent, $name, fmtx(&$svar)));
        )+
    };
}
#[macro_export]
macro_rules! MdlWriteAnimStatic {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr => $def:expr => $svar:expr ),+ $(,)?) => {
        $(if let None = &$avar && $svar != $def {
            _MdlWriteAnimStatic!($lines, $depth, $name => $svar);
        })+
    };
}
#[macro_export]
macro_rules! MdlWriteAnimBoth {
    ($lines:ident, $depth:expr, $( $name:expr => $avar:expr => $def:expr => $svar:expr ),+ $(,)?) => {
        $(if let Some(item) = &$avar {
            _MdlWriteAnim!($lines, $depth, $name => item);
        } else if $svar != $def {
            _MdlWriteAnimStatic!($lines, $depth, $name => $svar);
        })+
    };
}

//#region InterpolationType

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InterpolationType {
    #[default]
    DontInterp,
    Linear,
    Hermite,
    Bezier,
    Error(u32),
}
impl InterpolationType {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::DontInterp,
            1 => Self::Linear,
            2 => Self::Hermite,
            3 => Self::Bezier,
            x => Self::Error(x),
        }
    }
    fn has_tans(&self) -> bool {
        matches!(self, Self::Hermite | Self::Bezier)
    }
}

//#endregion
//#region formatter

fn fmt_key_frames<T: TAnimation>(key_frames: &Vec<KeyFrame<T>>) -> String {
    let mut list: Vec<String> = Vec::with_capacity(key_frames.len());
    for kf in key_frames {
        list.push(fmt_key_frame_1(kf));
    }
    format!("[\n    {}\n]", list.join("\n    "))
}
fn fmt_key_frame_1<T: TAnimation>(kf: &KeyFrame<T>) -> String {
    match kf.has_tans {
        true => format!("{}: {}, InTan={}, OutTan={},", kf.frame, fmtx(&kf.value), fmtx(&kf.itan), fmtx(&kf.otan)),
        false => format!("{}: {},", kf.frame, fmtx(&kf.value)),
    }
}

//#endregion
