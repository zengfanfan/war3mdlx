use crate::*;

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
    pub alpha_anim: Option<Animation<f32>>,
    pub texid_anim: Option<Animation<i32>>,
}

impl Material {
    pub const ID: u32 = MdlxMagic::MTLS as u32;

    // [todo] check flags
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

        this.priority_plane = cur.readx()?;
        this.flags = cur.readx()?;

        if cur.left() > 8 && cur.read_be::<u32>()? == Layer::ID {
            let count: i32 = cur.readx()?;
            for _ in 0..count {
                let sz: u32 = cur.readx()?;
                let body = cur.read_bytes(sz - 4)?;
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

    // [todo] check flags
    pub fn unshaded(&self) -> bool {
        return self.flags & 0x1 != 0;
    }
    pub fn sphere_env_map(&self) -> bool {
        return self.flags & 0x2 != 0;
    }
    pub fn two_sided(&self) -> bool {
        return self.flags & 0x10 != 0;
    }
    pub fn unfogged(&self) -> bool {
        return self.flags & 0x20 != 0;
    }
    pub fn no_depth_test(&self) -> bool {
        return self.flags & 0x40 != 0;
    }
    pub fn no_depth_set(&self) -> bool {
        return self.flags & 0x80 != 0;
    }

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.filter_mode = FilterMode::from(cur.readx()?);
        this.flags = cur.readx()?;
        this.texture_id = cur.readx()?;
        this.texture_anim_id = cur.readx()?;
        this.unknown_1 = cur.readx()?;
        this.alpha = cur.readx()?;

        while cur.left() > 16 {
            match cur.read_be()? {
                id @ Self::ID_ALPHA => this.alpha_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_TEXID => this.texid_anim = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
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
impl FilterMode {
    fn from(v: u32) -> FilterMode {
        match v {
            0 => FilterMode::None,
            1 => FilterMode::Transparent,
            2 => FilterMode::Blend,
            3 => FilterMode::Additive,
            4 => FilterMode::AddAlpha,
            5 => FilterMode::Modulate,
            6 => FilterMode::Modulate2X,
            7 => FilterMode::AlphaKey,
            x => FilterMode::Error(x),
        }
    }
}
