use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use derive_debug::Dbg;
use std::fmt::Debug as stdDebug;
use std::io::Cursor;

#[derive(Dbg, Default)]
pub struct KeyFrame<T: ReadFromCursor + stdDebug + Default> {
    pub frame: i32,
    pub value: T,
    pub itan: T,
    pub otan: T,
    #[dbg(skip)]
    pub has_tans: bool,
}

#[derive(Dbg, Default)]
pub struct Animation<T: ReadFromCursor + stdDebug + Default> {
    #[dbg(formatter = "fmt_id4s")]
    pub id: u32,
    pub interp_type: InterpolationType,
    pub global_seq_id: i32,
    #[dbg(formatter = "fmt_key_frames")]
    pub key_frames: Vec<KeyFrame<T>>,
}

impl<T: ReadFromCursor + stdDebug + Default> Animation<T> {
    fn read_mdx_value(cursor: &mut Cursor<&Vec<u8>>) -> Result<T, MyError> {
        Ok(T::read_from(cursor)?)
    }

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>, id: u32) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.id = id;
        let kfn = cur.read_i32::<LittleEndian>()?;
        this.interp_type = match cur.read_u32::<LittleEndian>()? {
            0 => InterpolationType::DontInterp,
            1 => InterpolationType::Linear,
            2 => InterpolationType::Hermite,
            3 => InterpolationType::Bezier,
            x => InterpolationType::Error(x),
        };
        this.global_seq_id = cur.read_i32::<LittleEndian>()?;

        for _ in 0..kfn {
            let frame = cur.read_i32::<LittleEndian>()?;
            let value = Self::read_mdx_value(cur)?;
            let has_tans = this.interp_type.has_tans();
            let itan = match has_tans {
                true => Self::read_mdx_value(cur)?,
                false => T::default(),
            };
            let otan = match has_tans {
                true => Self::read_mdx_value(cur)?,
                false => T::default(),
            };
            this.key_frames.push(KeyFrame { frame, value, itan, otan, has_tans });
        }

        return Ok(this);
    }
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
    fn has_tans(&self) -> bool {
        matches!(self, InterpolationType::Hermite | InterpolationType::Bezier)
    }
}

//#endregion
//#region formatter

fn fmt_key_frames<T: ReadFromCursor + stdDebug + Default>(key_frames: &Vec<KeyFrame<T>>) -> String {
    let mut list: Vec<String> = Vec::new();
    for kf in key_frames {
        list.push(fmt_key_frame_1(kf));
    }
    format!("[\n    {}\n]", list.join("\n    "))
}
fn fmt_key_frame_1<T: ReadFromCursor + stdDebug + Default>(kf: &KeyFrame<T>) -> String {
    match kf.has_tans {
        true => format!("{}: {:?}, InTan={:?}, OutTan={:?},", kf.frame, kf.value, kf.itan, kf.otan),
        false => format!("{}: {:?},", kf.frame, kf.value),
    }
}

//#endregion
//#region trait: ReadFromCursor

pub trait ReadFromCursor: Sized {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self>;
}

impl ReadFromCursor for i32 {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self> {
        cursor.read_i32::<LittleEndian>()
    }
}

impl ReadFromCursor for f32 {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self> {
        cursor.read_f32::<LittleEndian>()
    }
}

impl ReadFromCursor for Vec2 {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self> {
        Ok(Self::new(cursor.read_f32::<LittleEndian>()?, cursor.read_f32::<LittleEndian>()?))
    }
}

impl ReadFromCursor for Vec3 {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self> {
        Ok(Self::new(
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ))
    }
}

impl ReadFromCursor for Vec4 {
    fn read_from(cursor: &mut Cursor<&Vec<u8>>) -> ioResult<Self> {
        Ok(Self::new(
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        ))
    }
}

//#endregion
