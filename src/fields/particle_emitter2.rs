use crate::*;

#[derive(Dbg, Default)]
pub struct ParticleEmitter2 {
    pub base: Node,

    pub speed: f32,
    pub variation: f32,
    pub latitude: f32,
    pub gravity: f32,
    pub lifespan: f32,
    pub emit_rate: f32,
    pub length: f32,
    pub width: f32,

    pub filter_mode: PE2FilterMode,
    pub rows: i32,
    pub columns: i32,
    pub head_or_tail: HeadOrTail,

    pub tail_length: f32,
    pub time: f32,

    #[dbg(formatter = "fmtx")]
    pub segment_color: Vec<Vec3>,
    #[dbg(formatter = "fmtx")]
    pub segment_alpha: Vec<u8>, // 0~255
    #[dbg(formatter = "fmtx")]
    pub segment_scaling: Vec<f32>,

    pub head_life: PE2UVAnim,
    pub head_decay: PE2UVAnim,
    pub tail_life: PE2UVAnim,
    pub tail_decay: PE2UVAnim,

    pub texture_id: i32,
    pub squirt: bool,
    pub priority_plane: i32,
    pub replace_id: i32,

    #[dbg(formatter = "fmtxx")]
    pub speed_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub variation_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub latitude_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub gravity_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub emit_rate_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub length_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub width_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub visibility: Option<Animation<f32>>,
}

impl ParticleEmitter2 {
    pub const ID: u32 = MdlxMagic::PRE2 as u32;
    const ID_V: u32 = MdlxMagic::KP2V as u32; /* Visibility */
    const ID_ER: u32 = MdlxMagic::KP2E as u32; /* Emission Rate */
    const ID_W: u32 = MdlxMagic::KP2W as u32; /* Width */
    const ID_L: u32 = MdlxMagic::KP2N as u32; /* Length */
    const ID_SPD: u32 = MdlxMagic::KP2S as u32; /* Speed */
    const ID_LA: u32 = MdlxMagic::KP2L as u32; /* Latitude */
    const ID_VA: u32 = MdlxMagic::KP2R as u32; /* Variation */
    const ID_G: u32 = MdlxMagic::KP2G as u32; /* Gravity */

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Build!{ base: Node::read_mdx(cur)? };

        this.speed = cur.readx()?;
        this.variation = cur.readx()?;
        this.latitude = cur.readx()?;
        this.gravity = cur.readx()?;
        this.lifespan = cur.readx()?;
        this.emit_rate = cur.readx()?;
        this.length = cur.readx()?;
        this.width = cur.readx()?;

        this.filter_mode = PE2FilterMode::from(cur.readx()?);
        if let PE2FilterMode::Error(v) = this.filter_mode {
            return ERR!("Unknown filter mode: {}", v);
        }
        this.rows = cur.readx()?;
        this.columns = cur.readx()?;
        this.head_or_tail = HeadOrTail::from(cur.readx()?);
        if let HeadOrTail::Error(v) = this.head_or_tail {
            return ERR!("Unknown filter mode: {}", v);
        }

        this.tail_length = cur.readx()?;
        this.time = cur.readx()?;

        this.segment_color = cur.read_array(3)?;
        this.segment_alpha = cur.read_array(3)?;
        this.segment_scaling = cur.read_array(3)?;

        this.head_life = PE2UVAnim { start: cur.readx()?, end: cur.readx()?, repeat: cur.readx()? };
        this.head_decay = PE2UVAnim { start: cur.readx()?, end: cur.readx()?, repeat: cur.readx()? };
        this.tail_life = PE2UVAnim { start: cur.readx()?, end: cur.readx()?, repeat: cur.readx()? };
        this.tail_decay = PE2UVAnim { start: cur.readx()?, end: cur.readx()?, repeat: cur.readx()? };

        this.texture_id = cur.readx()?;
        this.squirt = 0i32 != cur.readx()?;
        this.priority_plane = cur.readx()?;
        this.replace_id = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                Self::ID_V => this.visibility = Some(Animation::read_mdx(cur)?),
                Self::ID_ER => this.emit_rate_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_W => this.width_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_L => this.length_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_SPD => this.speed_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_LA => this.latitude_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_VA => this.variation_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_G => this.gravity_anim = Some(Animation::read_mdx(cur)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Build!{ base: Node::read_mdl(block)? };
        this.base.flags.insert(NodeFlags::ParticleEmitter);

        this.segment_alpha = vec![255; 3];
        this.segment_scaling = vec![1.0; 3];
        this.segment_color = vec![Vec3::ONE; 3];
        let (mut head, mut tail) = (false, false);

        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "Speed" => this.speed = f.value.to(),
                "Variation" => this.variation = f.value.to(),
                "Latitude" => this.latitude = f.value.to(),
                "Gravity" => this.gravity = f.value.to(),
                "LifeSpan" => this.lifespan = f.value.to(),
                "EmissionRate" => this.emit_rate = f.value.to(),
                "Length" => this.length = f.value.to(),
                "Width" => this.width = f.value.to(),

                "Alpha" => this.segment_alpha = f.value.to(),
                "ParticleScaling" => this.segment_scaling = f.value.to(),
                "LifeSpanUVAnim" => this.head_life = PE2UVAnim::read_mdl(&f.value),
                "DecayUVAnim" => this.head_decay = PE2UVAnim::read_mdl(&f.value),
                "TailUVAnim" => this.tail_life = PE2UVAnim::read_mdl(&f.value),
                "TailDecayUVAnim" => this.tail_decay = PE2UVAnim::read_mdl(&f.value),

                "Rows" => this.rows = f.value.to(),
                "Columns" => this.columns = f.value.to(),
                "TailLength" => this.tail_length = f.value.to(),
                "Time" => this.time = f.value.to(),

                "TextureID" => this.texture_id = f.value.to(),
                "PriorityPlane" => this.priority_plane = f.value.to(),
                "ReplaceableId" => this.replace_id = f.value.to(),

                "Squirt" => this.squirt = true,
                "Head" => head = true,
                "Tail" => tail = true,
                "Both" => (head, tail) = (true, true),
                "SortPrimsFarZ" => this.base.flags.insert(NodeFlags::PE2SortPrimFarZ),
                "LineEmitter" => this.base.flags.insert(NodeFlags::LineEmitter),
                "ModelSpace" => this.base.flags.insert(NodeFlags::ModelSpace),
                "Unshaded" => this.base.flags.insert(NodeFlags::PE2Unshaded),
                "Unfogged" => this.base.flags.insert(NodeFlags::Unfogged),
                "XYQuad" => this.base.flags.insert(NodeFlags::XYQuad),
                _other => this.filter_mode = PE2FilterMode::from_str(f.name.as_str(), this.filter_mode),
            );
        }

        for b in &block.blocks {
            match_istr!(b.typ.as_str(),
                "SegmentColor" => this.segment_color = b.fields.to(),
                "Speed" => this.speed_anim = Some(Animation::read_mdl(b)?),
                "Variation" => this.variation_anim = Some(Animation::read_mdl(b)?),
                "Latitude" => this.latitude_anim = Some(Animation::read_mdl(b)?),
                "Gravity" => this.gravity_anim = Some(Animation::read_mdl(b)?),
                "EmissionRate" => this.emit_rate_anim = Some(Animation::read_mdl(b)?),
                "Length" => this.length_anim = Some(Animation::read_mdl(b)?),
                "Width" => this.width_anim = Some(Animation::read_mdl(b)?),
                "Visibility" => this.visibility = Some(Animation::read_mdl(b)?),
                _other => (),
            );
        }

        this.head_or_tail = yesno!(head && tail, HeadOrTail::Both, yesno!(tail, HeadOrTail::Tail, HeadOrTail::Head));
        this.segment_color = this.segment_color.convert(|a| a.reverse());

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];

        lines.append(&mut self.base.write_mdl(depth)?);

        lines.push(F!("{indent}{:?},", &self.filter_mode));
        {
            let mut clines: Vec<String> = vec![];
            for c in self.segment_color.iter() {
                let bgr = c.reverse();
                clines.pushx(&F!("{indent2}Color"), &bgr);
            }
            if !clines.is_empty() {
                lines.push(F!("{indent}SegmentColor {{"));
                lines.append(&mut clines);
                lines.push(F!("{indent}}},"));
            }
        }
        lines.pushx(&F!("{indent}Alpha"), &self.segment_alpha);
        lines.pushx(&F!("{indent}ParticleScaling"), &self.segment_scaling);
        lines.pushx_if_n0(&F!("{indent}LifeSpanUVAnim"), &self.head_life);
        lines.pushx_if_n0(&F!("{indent}DecayUVAnim"), &self.head_decay);
        lines.pushx_if_n0(&F!("{indent}TailUVAnim"), &self.tail_life);
        lines.pushx_if_n0(&F!("{indent}TailDecayUVAnim"), &self.tail_decay);

        lines.pushx_if_n0(&F!("{indent}Rows"), &self.rows);
        lines.pushx_if_n0(&F!("{indent}Columns"), &self.columns);
        lines.pushx_if_n0(&F!("{indent}Time"), &self.time);
        lines.pushx_if_n0(&F!("{indent}LifeSpan"), &self.lifespan);
        lines.pushx_if_n0(&F!("{indent}TailLength"), &self.tail_length);
        lines.pushx_if_nneg1(&F!("{indent}TextureID"), &self.texture_id);
        lines.pushx_if_n0(&F!("{indent}ReplaceableId"), &self.replace_id);
        lines.pushx_if_n0(&F!("{indent}PriorityPlane"), &self.priority_plane);

        let flags = self.base.flags;
        lines.push_if(flags.contains(NodeFlags::PE2SortPrimFarZ), F!("{indent}SortPrimsFarZ,"));
        lines.push_if(flags.contains(NodeFlags::LineEmitter), F!("{indent}LineEmitter,"));
        lines.push_if(flags.contains(NodeFlags::ModelSpace), F!("{indent}ModelSpace,"));
        lines.push_if(flags.contains(NodeFlags::PE2Unshaded), F!("{indent}Unshaded,"));
        lines.push_if(flags.contains(NodeFlags::Unfogged), F!("{indent}Unfogged,"));
        lines.push_if(flags.contains(NodeFlags::XYQuad), F!("{indent}XYQuad,"));
        lines.push_if(self.squirt, F!("{indent}Squirt,"));
        lines.push_if(self.head_or_tail.is_valid(), F!("{indent}{:?},", self.head_or_tail));

        MdlWriteAnimBoth!(lines, depth,
            "Speed" => self.speed_anim => 0.0 => self.speed,
            "Variation" => self.variation_anim => 0.0 => self.variation,
            "Latitude" => self.latitude_anim => 0.0 => self.latitude,
            "Gravity" => self.gravity_anim => 0.0 => self.gravity,
            "EmissionRate" => self.emit_rate_anim => 0.0 => self.emit_rate,
            "Length" => self.length_anim => 0.0 => self.length,
            "Width" => self.width_anim => 0.0 => self.width,
        );
        MdlWriteAnimIfSome!(lines, depth, "Visibility" => self.visibility);

        return Ok(lines);
    }
}

//#region PE2UVAnim

#[derive(Default)]
pub struct PE2UVAnim {
    pub start: i32,
    pub end: i32,
    pub repeat: i32,
}
impl PE2UVAnim {
    pub fn read_mdl(value: &MdlValue) -> Self {
        let mut this = Self { start: 0, end: 0, repeat: 1 };
        if let MdlValue::IntegerArray(iv) = value {
            yes!(iv.len() > 0, this.start = iv[0]);
            yes!(iv.len() > 1, this.end = iv[1]);
            yes!(iv.len() > 2, this.repeat = iv[2]);
        }
        return this;
    }
}

impl stdDebug for PE2UVAnim {
    fn fmt(&self, f: &mut stdFormatter<'_>) -> stdResult {
        write!(f, "({} ~ {}) x {}", self.start, self.end, self.repeat)
    }
}

impl CheckValue for PE2UVAnim {
    fn is0(&self) -> bool {
        (self.start, self.end, self.repeat) == (0, 0, 0)
    }
    fn is1(&self) -> bool {
        (self.start, self.end, self.repeat) == (1, 1, 1)
    }
    fn isneg1(&self) -> bool {
        (self.start, self.end, self.repeat) == (-1, -1, -1)
    }
}

impl Formatter for PE2UVAnim {
    fn fmt(&self) -> String {
        F!("{{ {}, {}, {} }}", self.start, self.end, self.repeat)
    }
}

//#endregion
//#region HeadOrTail

#[derive(Debug, Default)]
pub enum HeadOrTail {
    #[default]
    Head,
    Tail,
    Both,
    Error(i32),
}
impl HeadOrTail {
    fn is_valid(&self) -> bool {
        matches!(self, Self::Head | Self::Tail | Self::Both)
    }
    fn from(v: i32) -> Self {
        match v {
            0 => Self::Head,
            1 => Self::Tail,
            2 => Self::Both,
            x => Self::Error(x),
        }
    }
}

//#endregion
//#region PE2FilterMode

#[derive(Debug, Default)]
pub enum PE2FilterMode {
    #[default]
    Blend,
    Additive,
    Modulate,
    AlphaKey,
    Error(i32),
}
impl PE2FilterMode {
    fn from(v: i32) -> Self {
        match v {
            0 => Self::Blend,
            1 => Self::Additive,
            2 => Self::Modulate,
            4 => Self::AlphaKey,
            x => Self::Error(x),
        }
    }
    fn from_str(s: &str, def: Self) -> Self {
        match_istr!(s,
            "Blend" => Self::Blend,
            "Additive" => Self::Additive,
            "Modulate" => Self::Modulate,
            "AlphaKey" => Self::AlphaKey,
            _err => def,
        )
    }
}

//#endregion
