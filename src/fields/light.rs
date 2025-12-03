use crate::*;

#[derive(Dbg, Default)]
pub struct Light {
    pub base: Node,
    pub typ: LightType,
    pub attenuate_start: f32,
    pub attenuate_end: f32,
    #[dbg(formatter = "fmtx")]
    pub color: Vec3,
    pub intensity: f32,
    #[dbg(formatter = "fmtx")]
    pub ambient_color: Vec3,
    pub ambient_intensity: f32,
    pub visibility: Option<Animation<f32>>,
    pub attenuate_start_anim: Option<Animation<f32>>,
    pub attenuate_end_anim: Option<Animation<f32>>,
    pub color_anim: Option<Animation<Vec3>>,
    pub intensity_anim: Option<Animation<f32>>,
    pub ambient_color_anim: Option<Animation<Vec3>>,
    pub ambient_intensity_anim: Option<Animation<f32>>,
}

impl Light {
    pub const ID: u32 = MdlxMagic::LITE as u32;
    const ID_V: u32 = MdlxMagic::KLAV as u32; /* Visibility */
    const ID_AS: u32 = MdlxMagic::KLAS as u32; /* Attenuate start */
    const ID_AE: u32 = MdlxMagic::KLAE as u32; /* Attenuate end */
    const ID_C: u32 = MdlxMagic::KLAC as u32; /* Color */
    const ID_I: u32 = MdlxMagic::KLAI as u32; /* Intensity */
    const ID_AC: u32 = MdlxMagic::KLBC as u32; /* Ambient color */
    const ID_AI: u32 = MdlxMagic::KLBI as u32; /* Ambient intensity */
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::parse_mdx(cur)?;
        this.typ = LightType::from(cur.readx()?);
        if let LightType::Error(v) = this.typ {
            return ERR!("Unknown light type: {}", v);
        }

        this.attenuate_start = cur.readx()?;
        this.attenuate_end = cur.readx()?;
        this.color = cur.readx()?;
        this.intensity = cur.readx()?;
        this.ambient_color = cur.readx()?;
        this.ambient_intensity = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_V => this.visibility = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_AS => this.attenuate_start_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_AE => this.attenuate_end_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_C => this.color_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_I => this.intensity_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_AC => this.ambient_color_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_AI => this.ambient_intensity_anim = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum LightType {
    Omnidirectional = 0,
    Directional = 1,
    Ambient = 2,
    Error(u32),
}
impl Default for LightType {
    fn default() -> Self {
        LightType::Omnidirectional
    }
}
impl LightType {
    fn from(v: u32) -> LightType {
        match v {
            0 => LightType::Omnidirectional,
            1 => LightType::Directional,
            2 => LightType::Ambient,
            _ => LightType::Error(v),
        }
    }
}
