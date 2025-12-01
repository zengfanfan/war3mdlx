use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use derive_debug::Dbg;

#[derive(Dbg, Default)]
pub struct Texture {
    pub replaceable_id: u32,
    pub filename: String,
    #[dbg(skip)]
    pub unknown_1: u32,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: u32,
}

impl Texture {
    pub const ID: u32 = MdlxMagic::TEXS as u32;
    pub const NAME_SIZE: usize = 256;

    pub fn wrap_width(&self) -> bool {
        self.flags & 0x1 == 0x1
    }
    pub fn wrap_height(&self) -> bool {
        self.flags & 0x2 == 0x2
    }

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.replaceable_id = cur.read_u32::<LittleEndian>()?;

        let mut buf = vec![0u8; Self::NAME_SIZE];
        cur.read_exact(&mut buf)?;
        while buf.last() == Some(&0) {
            buf.pop(); // trim trailing \0
        }
        this.filename = String::from_utf8(buf).expect("Invalid UTF-8");

        this.unknown_1 = cur.read_u32::<LittleEndian>()?;
        this.flags = cur.read_u32::<LittleEndian>()?;
        return Ok(this);
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
    pub const ID_T: u32 = MdlxMagic::KTAT as u32;
    pub const ID_R: u32 = MdlxMagic::KTAR as u32;
    pub const ID_S: u32 = MdlxMagic::KTAS as u32;

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        while cur.position() + 16 < cur.get_ref().len() as u64 {
            let id = cur.read_u32::<BigEndian>()?;
            if id == Self::ID_T {
                this.translation = Some(Animation::parse_mdx(cur, id)?);
            } else if id == Self::ID_R {
                this.rotation = Some(Animation::parse_mdx(cur, id)?);
            } else if id == Self::ID_S {
                this.scaling = Some(Animation::parse_mdx(cur, id)?);
            } else {
                return Err(MyError::String("Unknown animation type".to_string()));
            }
        }

        return Ok(this);
    }
}
