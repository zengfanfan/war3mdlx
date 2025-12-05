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

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
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
                id @ Self::ID_V => this.visibility = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_AS => this.attenuate_start_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_AE => this.attenuate_end_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_C => this.color_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_I => this.intensity_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_AC => this.ambient_color_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_AI => this.ambient_intensity_anim = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];

        lines.append(&mut self.base.write_mdl(depth)?);
        lines.push(F!("{indent}{:?},", self.typ));

        let (bgr, bgr2) = (self.color.reverse(), self.ambient_color.reverse());
        let bgr_anim = self.color_anim.as_ref().and_then(|a| Some(a.convert(|v| v.reverse())));
        let bgr2_anim = self.ambient_color_anim.as_ref().and_then(|a| Some(a.convert(|v| v.reverse())));

        MdlWriteAnimStatic!(lines, depth,
            "AttenuationStart" => self.attenuate_start_anim => 0.0 => self.attenuate_start,
            "AttenuationEnd" => self.attenuate_end_anim => 0.0 => self.attenuate_end,
            "Color" => bgr_anim => Vec3::NEG_ONE => bgr,
            "Intensity" => self.intensity_anim => 0.0 => self.intensity,
            "AmbColor" => bgr2_anim => Vec3::NEG_ONE => bgr2,
            "AmbIntensity" => self.ambient_intensity_anim => 0.0 => self.ambient_intensity,
        );
        MdlWriteAnim!(lines, depth,
            "AttenuationStart" => self.attenuate_start_anim,
            "AttenuationEnd" => self.attenuate_end_anim,
            "Color" => bgr_anim,
            "Intensity" => self.intensity_anim,
            "AmbColor" => bgr2_anim,
            "AmbIntensity" => self.ambient_intensity_anim,
            "Visibility" => self.visibility,
        );

        return Ok(lines);
    }
}

#[repr(u32)]
#[derive(Debug, Default)]
pub enum LightType {
    #[default]
    Omnidirectional = 0,
    Directional = 1,
    Ambient = 2,
    Error(u32),
}
impl LightType {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::Omnidirectional,
            1 => Self::Directional,
            2 => Self::Ambient,
            _ => Self::Error(v),
        }
    }
}
