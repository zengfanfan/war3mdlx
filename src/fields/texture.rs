use crate::*;

#[derive(Dbg, Default)]
pub struct Texture {
    pub replace_id: i32,
    pub path: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub flags: TextureFlags,
}
bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureFlags : u32 {
        const WrapWidth = 1 << 0;
        const WrapHeight = 1 << 1;
    }
}

impl Texture {
    pub const ID: u32 = MdlxMagic::TEXS as u32;
    const PATH_SIZE: u32 = 256;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            replace_id: cur.readx()?,
            path: cur.read_string(Self::PATH_SIZE)?,
            _unknown: cur.readx()?,
            flags: TextureFlags::from_bits_retain(cur.readx::<u32>()?),
        })
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}Bitmap {{"));
        lines.push_if_n0(&F!("{indent2}ReplaceableId"), &self.replace_id);
        yes!(self.replace_id.is0(), lines.push(F!("{indent2}Image \"{}\",", self.path)));
        yes!(self.flags.contains(TextureFlags::WrapWidth), lines.push(F!("{indent2}WrapWidth,")));
        yes!(self.flags.contains(TextureFlags::WrapHeight), lines.push(F!("{indent2}WrapHeight,")));
        lines.push(F!("{indent}}}"));
        return Ok(lines);
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

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_T => this.translation = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_R => this.rotation = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_S => this.scaling = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}TVertexAnim {{"));
        MdlWriteAnim!(lines, 2,
            "Translation" => self.translation,
            "Rotation" => self.rotation,
            "Scaling" => self.scaling,
        );
        lines.push(F!("{indent}}}"));
        return Ok(lines);
    }
}
