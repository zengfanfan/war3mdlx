use crate::*;
use byteorder::{BigEndian,LittleEndian, ReadBytesExt};
use derive_debug::Dbg;
use std::io::{Cursor, Read};

#[derive(Dbg, Default)]
pub struct Material {
    pub priority_plane: u32,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: u32,
    pub layers: Vec<Layer>,
}

#[derive(Dbg, Default)]
pub struct Layer {
    pub filter_mode: FilterMode,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: u32,
    pub texture_id: i32,
    pub texture_anim_id: i32,
    #[dbg(skip)]
    pub unknown_1: u32,
    pub alpha: f32,
    // pub alpha_anim: Animation<f32>,
    // pub texid_anim: Animation<f32>,
}

impl Material {
    pub const ID: u32 = MdlxMagic::MTLS as u32;

    pub fn contant_color(&self) -> bool {
        return self.flags & 0x1 != 0;
    }
    pub fn sort_primitives_far_z(&self) -> bool {
        return self.flags & 0x10 != 0;
    }
    pub fn full_resolution(&self) -> bool {
        return self.flags & 0x20 != 0;
    }

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.priority_plane = cur.read_u32::<LittleEndian>()?;
        this.flags = cur.read_u32::<LittleEndian>()?;

        let anim_len = cur.get_ref().len() as u64 - cur.position();
        if anim_len>8 && cur.read_u32::<BigEndian>()? == Layer::ID {
            let count = cur.read_i32::<LittleEndian>()?;
            for _ in 0..count {
                let sz = cur.read_u32::<LittleEndian>()? -4;
                let mut body = vec![0u8; sz as usize];
                cur.read_exact(&mut body)?;
                let mut cur2 = Cursor::new(&body);
                this.layers.push(Layer::parse_mdx(&mut cur2)?);
            }
        }

        return Ok(this);
    }
}

impl Layer {
    pub const ID: u32 = MdlxMagic::LAYS as u32;
    pub const ID_ALPHA: u32 = MdlxMagic::KMTA as u32;
    pub const ID_TEXID: u32 = MdlxMagic::KMTF as u32;

    pub fn xxx(&self) -> bool {
        return self.flags & 0x1 != 0;
    }

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.filter_mode = match cur.read_u32::<LittleEndian>()? {
            0 => FilterMode::None,
            1 => FilterMode::Transparent,
            2 => FilterMode::Blend,
            3 => FilterMode::Additive,
            4 => FilterMode::AddAlpha,
            5 => FilterMode::Modulate,
            6 => FilterMode::Modulate2X,
            7 => FilterMode::AlphaKey,
            x => FilterMode::Error(x),
        };
        this.flags = cur.read_u32::<LittleEndian>()?;
        this.texture_id = cur.read_i32::<LittleEndian>()?;
        this.texture_anim_id = cur.read_i32::<LittleEndian>()?;
        this.unknown_1 = cur.read_u32::<LittleEndian>()?;
        this.alpha = cur.read_f32::<LittleEndian>()?;

        while cur.position() < cur.get_ref().len() as u64 {
            let id = cur.read_u32::<BigEndian>()?;
            let kfn = cur.read_i32::<LittleEndian>()?;

            // is intan/outtan always there? [?]

            // let chunk = MdlxChunk::parse_mdx(cur)?;
            // if chunk.id == Layer::ID_ALPHA {
            //     let mut _cur = Cursor::new(&chunk.body);
            //         this.alpha_anim = Animation<f32>::parse_mdx(&mut _cur)?;
            // } else if chunk.id == Layer::ID_ALPHA {
            //     let mut _cur = Cursor::new(&chunk.body);
            //         this.texid_anim = Animation<f32>::parse_mdx(&mut _cur)?;
            // }
        }

        return Ok(this);
    }
}

#[derive(Debug)]
pub enum FilterMode {
    None,
    Transparent,
    Blend,
    Additive,
    AddAlpha,
    Modulate,
    Modulate2X,
    AlphaKey,
    Error(u32),
}
impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::None
    }
}
