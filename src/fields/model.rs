use crate::*;

#[derive(Dbg, Default)]
pub struct Model {
    pub name: String,
    #[dbg(skip)]
    pub unknown_1: u32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub minimum_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub maximum_extent: Vec3,
    pub blend_time: u32,
}

impl Model {
    pub const ID: u32 = MdlxMagic::MODL as u32;
    pub const NAME_SIZE: u32 = 336;

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            name: cur.read_string(Self::NAME_SIZE)?,
            unknown_1: cur.readx()?,
            bounds_radius: cur.readx()?,
            minimum_extent: cur.readx()?,
            maximum_extent: cur.readx()?,
            blend_time: cur.readx()?,
        })
    }
}
