use crate::*;

#[derive(Dbg, Default)]
pub struct Light {
    pub base: Node,

    #[dbg(fmt = "{:?}")]
    pub typ: LightType,
    pub attenuate_start: f32,
    pub attenuate_end: f32,
    #[dbg(formatter = "fmtx")]
    pub color: Vec3,
    pub intensity: f32,
    #[dbg(formatter = "fmtx")]
    pub ambient_color: Vec3,
    pub ambient_intensity: f32,

    #[dbg(formatter = "fmtxx")]
    pub attenuate_start_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub attenuate_end_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub color_anim: Option<Animation<Vec3>>,
    #[dbg(formatter = "fmtxx")]
    pub intensity_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub ambient_color_anim: Option<Animation<Vec3>>,
    #[dbg(formatter = "fmtxx")]
    pub ambient_intensity_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub visibility: Option<Animation<f32>>,
}

impl Light {
    pub const ID: u32 = MdlxMagic::LITE as u32;
    const ID_AS: u32 = MdlxMagic::KLAS as u32; /* Attenuate start */
    const ID_AE: u32 = MdlxMagic::KLAE as u32; /* Attenuate end */
    const ID_C: u32 = MdlxMagic::KLAC as u32; /* Color */
    const ID_I: u32 = MdlxMagic::KLAI as u32; /* Intensity */
    const ID_AC: u32 = MdlxMagic::KLBC as u32; /* Ambient color */
    const ID_AI: u32 = MdlxMagic::KLBI as u32; /* Ambient intensity */
    const ID_V: u32 = MdlxMagic::KLAV as u32; /* Visibility */

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Build! { base: Node::read_mdx(cur)? };

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
                Self::ID_AS => this.attenuate_start_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_AE => this.attenuate_end_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_C => this.color_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_I => this.intensity_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_AC => this.ambient_color_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_AI => this.ambient_intensity_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_V => this.visibility = Some(Animation::read_mdx(cur)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdx(&self, chunk: &mut MdxChunk) -> Result<(), MyError> {
        chunk.write(&self.calc_mdx_size())?;
        self.base.write_mdx(chunk)?;

        chunk.write(&self.typ.to())?;
        chunk.write(&self.attenuate_start)?;
        chunk.write(&self.attenuate_end)?;
        chunk.write(&self.color)?;
        chunk.write(&self.intensity)?;
        chunk.write(&self.ambient_color)?;
        chunk.write(&self.ambient_intensity)?;

        MdxWriteAnim!(chunk,
            Self::ID_AS => self.attenuate_start_anim,
            Self::ID_AE => self.attenuate_end_anim,
            Self::ID_C  => self.color_anim,
            Self::ID_I  => self.intensity_anim,
            Self::ID_AC => self.ambient_color_anim,
            Self::ID_AI => self.ambient_intensity_anim,
            Self::ID_V  => self.visibility,
        );
        return Ok(());
    }
    pub fn calc_mdx_size(&self) -> u32 {
        let mut sz: u32 = 48; // sz + type + as + ae + color + intensity + amb_color + amb_intensity
        sz += self.base.calc_mdx_size();
        sz += self.attenuate_start_anim.calc_mdx_size();
        sz += self.attenuate_end_anim.calc_mdx_size();
        sz += self.color_anim.calc_mdx_size();
        sz += self.intensity_anim.calc_mdx_size();
        sz += self.ambient_color_anim.calc_mdx_size();
        sz += self.ambient_intensity_anim.calc_mdx_size();
        sz += self.visibility.calc_mdx_size();
        return sz;
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Build! { base: Node::read_mdl(block)? };
        this.base.flags.insert(NodeFlags::Light);

        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "AttenuationStart" => this.attenuate_start = f.value.to(),
                "AttenuationEnd" => this.attenuate_end = f.value.to(),
                "Color" => this.color = f.value.to(),
                "Intensity" => this.intensity = f.value.to(),
                "AmbColor" => this.ambient_color = f.value.to(),
                "AmbIntensity" => this.ambient_intensity = f.value.to(),
                _other => this.typ = LightType::from_str(_other, this.typ),
            );
        }

        for b in &block.blocks {
            match_istr!(b.typ.as_str(),
                "AttenuationStart" => this.attenuate_start_anim = Some(Animation::read_mdl(b)?),
                "AttenuationEnd" => this.attenuate_end_anim = Some(Animation::read_mdl(b)?),
                "Color" => this.color_anim = Some(Animation::read_mdl(b)?),
                "Intensity" => this.intensity_anim = Some(Animation::read_mdl(b)?),
                "AmbColor" => this.ambient_color_anim = Some(Animation::read_mdl(b)?),
                "AmbIntensity" => this.ambient_intensity_anim = Some(Animation::read_mdl(b)?),
                "Visibility" => this.visibility = Some(Animation::read_mdl(b)?),
                _other => (),
            );
        }

        this.color = this.color.reverse();
        this.ambient_color = this.ambient_color.reverse();
        this.color_anim = this.color_anim.map(|a| a.convert(|v| v.reverse()));
        this.ambient_color_anim = this.ambient_color_anim.map(|a| a.convert(|v| v.reverse()));

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

        MdlWriteAnimBoth!(lines, depth,
            "AttenuationStart" => self.attenuate_start_anim => 0.0 => self.attenuate_start,
            "AttenuationEnd" => self.attenuate_end_anim => 0.0 => self.attenuate_end,
            "Color" => bgr_anim => Vec3::ZERO => bgr,
            "Intensity" => self.intensity_anim => 0.0 => self.intensity,
            "AmbColor" => bgr2_anim => Vec3::ZERO => bgr2,
            "AmbIntensity" => self.ambient_intensity_anim => 0.0 => self.ambient_intensity,
        );
        MdlWriteAnimIfSome!(lines, depth, "Visibility" => self.visibility);

        return Ok(lines);
    }
}

//#region LightType

#[derive(Debug, Default)]
pub enum LightType {
    #[default]
    Omnidirectional,
    Directional,
    Ambient,
    Error(i32),
}
impl LightType {
    fn from(v: i32) -> Self {
        match v {
            0 => Self::Omnidirectional,
            1 => Self::Directional,
            2 => Self::Ambient,
            _ => Self::Error(v),
        }
    }
    fn from_str(s: &str, def: Self) -> Self {
        match_istr!(s,
            "Omnidirectional" => Self::Omnidirectional,
            "Directional" => Self::Directional,
            "Ambient" => Self::Ambient,
            _err => def,
        )
    }

    fn to(&self) -> i32 {
        match self {
            Self::Omnidirectional => 0,
            Self::Directional => 1,
            Self::Ambient => 2,
            Self::Error(v) => *v,
        }
    }
}

//#endregion
