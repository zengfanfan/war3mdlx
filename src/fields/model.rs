use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use derive_debug::Dbg;
use std::io::{Cursor, Read};

#[derive(Dbg, Default)]
pub struct Model {
    pub name: String,
    #[dbg(skip)]
    pub unknown_1: u32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmt_vec3")]
    pub minimum_extent: Vec3,
    #[dbg(formatter = "fmt_vec3")]
    pub maximum_extent: Vec3,
    pub blend_time: u32,
}

impl Model {
    pub const ID: u32 = MdlxMagic::MODL as u32;
    pub const NAME_SIZE: usize = 336;

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        let mut buf = vec![0u8; Self::NAME_SIZE];
        cur.read_exact(&mut buf)?;
        while buf.last() == Some(&0) {
            buf.pop(); // trim trailing \0
        }
        this.name = String::from_utf8(buf).expect("Invalid UTF-8");
        this.unknown_1 = cur.read_u32::<LittleEndian>()?;
        this.bounds_radius = cur.read_f32::<LittleEndian>()?;
        this.minimum_extent = Vec3::new(
            cur.read_f32::<LittleEndian>()?,
            cur.read_f32::<LittleEndian>()?,
            cur.read_f32::<LittleEndian>()?,
        );
        this.maximum_extent = Vec3::new(
            cur.read_f32::<LittleEndian>()?,
            cur.read_f32::<LittleEndian>()?,
            cur.read_f32::<LittleEndian>()?,
        );
        this.blend_time = cur.read_u32::<LittleEndian>()?;
        return Ok(this);
    }
}
