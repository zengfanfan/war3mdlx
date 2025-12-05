use crate::*;

#[derive(Dbg, Default)]
pub struct Camera {
    pub name: String,
    #[dbg(formatter = "fmtx")]
    pub position: Vec3,
    pub field_of_view: f32,
    pub far_clip: f32,
    pub near_clip: f32,
    #[dbg(formatter = "fmtx")]
    pub target: Vec3,
    pub translation: Option<Animation<Vec3>>,
    pub rotation: Option<Animation<f32>>,
    pub target_translation: Option<Animation<Vec3>>,
}

impl Camera {
    pub const ID: u32 = MdlxMagic::CAMS as u32;
    const ID_T: u32 = MdlxMagic::KCTR as u32; /* Translation */
    const ID_R: u32 = MdlxMagic::KCRL as u32; /* Rotation (radians) */
    const ID_TT: u32 = MdlxMagic::KTTR as u32; /* Target translation */
    const NAME_SIZE: u32 = 80;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.name = cur.read_string(Self::NAME_SIZE)?;
        this.position = cur.readx()?;
        this.field_of_view = cur.readx()?;
        this.far_clip = cur.readx()?;
        this.near_clip = cur.readx()?;
        this.target = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_T => this.translation = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_R => this.rotation = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_TT => this.target_translation = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];

        lines.pushx_if_n0(&F!("{indent}Position"), &self.position);
        lines.pushx_if_n0(&F!("{indent}FieldOfView"), &self.field_of_view);
        lines.pushx_if_n0(&F!("{indent}FarClip"), &self.far_clip);
        lines.pushx_if_n0(&F!("{indent}NearClip"), &self.near_clip);
        MdlWriteAnim!(lines, depth,
            "Translation" => self.translation,
            "Rotation" => self.rotation,
        );

        {
            let mut tines: Vec<String> = vec![];
            tines.pushx_if_n0(&F!("{indent2}Position"), &self.position);
            MdlWriteAnim!(tines, depth + 1, "Translation" => self.target_translation);
            if !tines.is_empty() {
                lines.push(F!("{indent}Target {{"));
                lines.append(&mut tines);
                lines.push(F!("{indent}}}"));
            }
        }

        return Ok(lines);
    }
}
