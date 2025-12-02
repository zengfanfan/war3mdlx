use crate::*;

#[derive(Dbg, Default)]
pub struct Sequence {
    pub name: String,
    pub start_frame: u32,
    pub end_frame: u32,
    pub move_speed: f32,
    pub looping: bool,
    pub rarity: f32,
    #[dbg(skip)]
    pub unknown_1: u32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub minimum_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub maximum_extent: Vec3,
}

impl Sequence {
    pub const ID: u32 = MdlxMagic::SEQS as u32;
    const NAME_SIZE: u32 = 80;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            name: cur.read_string(Self::NAME_SIZE)?,
            start_frame: cur.readx()?,
            end_frame: cur.readx()?,
            move_speed: cur.readx()?,
            looping: cur.readx::<u32>()? == 0,
            rarity: cur.readx()?,
            unknown_1: cur.readx()?,
            bounds_radius: cur.readx()?,
            minimum_extent: cur.readx()?,
            maximum_extent: cur.readx()?,
        })
    }
}
