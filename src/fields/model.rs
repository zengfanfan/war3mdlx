use crate::*;

#[derive(Dbg, Default)]
pub struct Model {
    pub name: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub minimum_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub maximum_extent: Vec3,
    pub blend_time: u32,
}

impl Model {
    pub const ID: u32 = MdlxMagic::MODL as u32;
    const NAME_SIZE: u32 = 336;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            name: cur.read_string(Self::NAME_SIZE)?,
            _unknown: cur.readx()?,
            bounds_radius: cur.readx()?,
            minimum_extent: cur.readx()?,
            maximum_extent: cur.readx()?,
            blend_time: cur.readx()?,
        })
    }
}
