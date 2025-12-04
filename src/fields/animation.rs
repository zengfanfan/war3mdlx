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
        for kf in &self.key_frames {
            lines.push(F!("{indent}{}: {},", kf.frame, fmtx(&kf.value)));
            if kf.has_tans {
                lines.push(F!("{indent2}InTan {},", fmtx(&kf.itan)));
                lines.push(F!("{indent2}OutTan {},", fmtx(&kf.otan)));
            }
        }
        return Ok(lines);
    }
}

#[macro_export]
macro_rules! MdlWriteAnim {
    ($lines:ident, $depth:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(
            if let Some(a) = &$var {
                let indent = indent!(&$depth);
                $lines.push(F!("{}{} {} {{", indent, $name, a.key_frames.len()));
                $lines.append(a.write_mdl($depth + 1)?.as_mut());
                $lines.push(F!("{}}}", indent));
            }
        )+
    };
}

//#region InterpolationType

#[derive(Debug)]
pub enum InterpolationType {
    DontInterp,
    Linear,
    Hermite,
    Bezier,
    Error(u32),
}
impl Default for InterpolationType {
    fn default() -> Self {
        InterpolationType::DontInterp
    }
}
impl InterpolationType {
    fn from(v: u32) -> InterpolationType {
        match v {
            0 => InterpolationType::DontInterp,
            1 => InterpolationType::Linear,
            2 => InterpolationType::Hermite,
            3 => InterpolationType::Bezier,
            x => InterpolationType::Error(x),
        }
    }
    fn has_tans(&self) -> bool {
        matches!(self, InterpolationType::Hermite | InterpolationType::Bezier)
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
