use crate::*;

#[derive(Dbg, Default)]
pub struct Texture {
    pub replaceable_id: i32,
    pub filename: String,
    #[dbg(skip)]
    pub _unknown: i32,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: u32,
}

impl Texture {
    pub const ID: u32 = MdlxMagic::TEXS as u32;
    const NAME_SIZE: u32 = 256;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            replaceable_id: cur.readx()?,
            filename: cur.read_string(Self::NAME_SIZE)?,
            _unknown: cur.readx()?,
            flags: cur.readx()?,
        })
    }
}

#[derive(Dbg, Default)]
pub struct TextureAnim {
    pub translation: Option<Animation<Vec3>>,
    pub rotation: Option<Animation<Vec4>>,
    pub scaling: Option<Animation<Vec3>>,
}

impl TextureAnim {
    pub const ID: u32 = MdlxMagic::TXAN as u32;
    const ID_T: u32 = MdlxMagic::KTAT as u32; /* Translation */
    const ID_R: u32 = MdlxMagic::KTAR as u32; /* Rotation */
    const ID_S: u32 = MdlxMagic::KTAS as u32; /* Scaling */
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_T => this.translation = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_R => this.rotation = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_S => this.scaling = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }
        return Ok(this);
    }
}
