use crate::*;

#[derive(Dbg, Default)]
pub struct Material {
    pub priority_plane: i32,
    pub flags: MaterialFlags,
    pub layers: Vec<Layer>,
}
bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MaterialFlags : u32 {
        const ConstantColor = 1 << 0;
        const SortPrimsFarZ = 1 << 4;
        const FullResolution = 1 << 5;
    }
}

#[derive(Dbg, Default)]
pub struct Layer {
    pub filter_mode: FilterMode,
    pub flags: LayerFlags,
    pub texture_id: i32,
    pub texture_anim_id: i32,
    #[dbg(skip)]
    pub _unknown: i32,
    pub alpha: f32,
    pub alpha_anim: Option<Animation<f32>>,
    pub texid_anim: Option<Animation<i32>>,
}
bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayerFlags : u32 {
        const Unshaded = 1 << 0;
        const SphereEnvMap = 1 << 1;
        const TwoSided = 1 << 4;
        const Unfogged = 1 << 5;
        const NoDepthTest = 1 << 6;
        const NoDepthSet = 1 << 7;
    }
}

impl Material {
    pub const ID: u32 = MdlxMagic::MTLS as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.priority_plane = cur.readx()?;
        this.flags = MaterialFlags::from_bits_retain(cur.readx()?);

        if cur.left() > 8 && cur.read_be::<u32>()? == Layer::ID {
            let count: i32 = cur.readx()?;
            for _ in 0..count {
                let sz: u32 = cur.readx()?;
                yes!(sz < 4, EXIT!("{} layer size: {} (need >= 4)", TNAME!(), sz));
                let body = cur.read_bytes(sz - 4)?;
                let mut cur2 = Cursor::new(&body);
                this.layers.push(Layer::read_mdx(&mut cur2)?);
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}Material {{"));
        lines.pushx_if_n0(&F!("{indent2}PriorityPlane"), &self.priority_plane);
        yes!(self.flags.contains(MaterialFlags::ConstantColor), lines.push(F!("{indent2}ConstantColor,")));
        yes!(self.flags.contains(MaterialFlags::SortPrimsFarZ), lines.push(F!("{indent2}SortPrimsFarZ,")));
        yes!(self.flags.contains(MaterialFlags::FullResolution), lines.push(F!("{indent2}FullResolution,")));

        for layer in &self.layers {
            lines.append(&mut layer.write_mdl(2)?);
        }

        lines.push(F!("{indent}}}"));
        return Ok(lines);
    }
}

impl Layer {
    pub const ID: u32 = MdlxMagic::LAYS as u32;
    const ID_ALPHA: u32 = MdlxMagic::KMTA as u32;
    const ID_TEXID: u32 = MdlxMagic::KMTF as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.filter_mode = FilterMode::from(cur.readx()?);
        if let FilterMode::Error(v) = this.filter_mode {
            EXIT!("Unknown filter mode: {}", v);
        }

        this.flags = LayerFlags::from_bits_retain(cur.readx()?);
        this.texture_id = cur.readx()?;
        this.texture_anim_id = cur.readx()?;
        this._unknown = cur.readx()?;
        this.alpha = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_ALPHA => this.alpha_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_TEXID => this.texid_anim = Some(Animation::read_mdx(cur, id)?),
                id => EXIT!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}Layer {{"));
        lines.push(F!("{indent2}FilterMode {:?},", self.filter_mode));

        yes!(self.flags.contains(LayerFlags::TwoSided), lines.push(F!("{indent2}TwoSided,")));
        yes!(self.flags.contains(LayerFlags::Unshaded), lines.push(F!("{indent2}Unshaded,")));
        yes!(self.flags.contains(LayerFlags::Unfogged), lines.push(F!("{indent2}Unfogged,")));
        yes!(self.flags.contains(LayerFlags::SphereEnvMap), lines.push(F!("{indent2}SphereEnvMap,")));
        yes!(self.flags.contains(LayerFlags::NoDepthTest), lines.push(F!("{indent2}NoDepthTest,")));
        yes!(self.flags.contains(LayerFlags::NoDepthSet), lines.push(F!("{indent2}NoDepthSet,")));
        lines.push_if_nneg1(&F!("{indent2}TVertexAnimId"), &self.texture_anim_id);

        MdlWriteAnimBoth!(lines, depth + 1,
            "Alpha" => self.alpha_anim => 1.0 => self.alpha,
            "TextureID" => self.texid_anim => -1 => self.texture_id,
        );

        lines.push(F!("{indent}}}"));
        return Ok(lines);
    }
}

#[derive(Debug, Default)]
pub enum FilterMode {
    #[default]
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
impl FilterMode {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Transparent,
            2 => Self::Blend,
            3 => Self::Additive,
            4 => Self::AddAlpha,
            5 => Self::Modulate,
            6 => Self::Modulate2X,
            7 => Self::AlphaKey,
            x => Self::Error(x),
        }
    }
}
