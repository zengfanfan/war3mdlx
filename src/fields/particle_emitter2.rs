use crate::*;

#[derive(Dbg, Default)]
pub struct ParticleEmitter2 {
    pub base: Node,

    pub speed: f32,
    pub variation: f32,
    pub vatitude: f32,
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

    pub visibility: Option<Animation<f32>>,
    pub emit_rate_anim: Option<Animation<f32>>,
    pub width_anim: Option<Animation<f32>>,
    pub length_anim: Option<Animation<f32>>,
    pub speed_anim: Option<Animation<f32>>,
    pub latitude_anim: Option<Animation<f32>>,
    pub variation_anim: Option<Animation<f32>>,
    pub gravity_anim: Option<Animation<f32>>,
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
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;

        this.speed = cur.readx()?;
        this.variation = cur.readx()?;
        this.vatitude = cur.readx()?;
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
                id @ Self::ID_V => this.visibility = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_ER => this.emit_rate_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_W => this.width_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_L => this.length_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_SPD => this.speed_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_LA => this.latitude_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_VA => this.variation_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_G => this.gravity_anim = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}

//#region PE2UVAnim

#[derive(Default)]
pub struct PE2UVAnim {
    pub start: i32,
    pub end: i32,
    pub repeat: i32,
}
impl stdDebug for PE2UVAnim {
    fn fmt(&self, f: &mut stdFormatter<'_>) -> stdResult {
        write!(f, "({} ~ {}) x {}", self.start, self.end, self.repeat)
    }
}

//#endregion
//#region PE2FilterMode

#[repr(u32)]
#[derive(Debug)]
pub enum HeadOrTail {
    Head = 0,
    Tail = 1,
    Both = 2,
    Error(u32),
}
impl Default for HeadOrTail {
    fn default() -> Self {
        HeadOrTail::Head
    }
}
impl HeadOrTail {
    fn from(v: u32) -> HeadOrTail {
        match v {
            0 => HeadOrTail::Head,
            1 => HeadOrTail::Tail,
            2 => HeadOrTail::Both,
            x => HeadOrTail::Error(x),
        }
    }
}

//#endregion
//#region PE2FilterMode

#[repr(u32)]
#[derive(Debug)]
pub enum PE2FilterMode {
    Blend = 0,
    Additive = 1,
    Modulate = 2,
    AlphaKey = 4,
    Error(u32),
}
impl Default for PE2FilterMode {
    fn default() -> Self {
        PE2FilterMode::Blend
    }
}
impl PE2FilterMode {
    fn from(v: u32) -> PE2FilterMode {
        match v {
            0 => PE2FilterMode::Blend,
            1 => PE2FilterMode::Additive,
            2 => PE2FilterMode::Modulate,
            4 => PE2FilterMode::AlphaKey,
            x => PE2FilterMode::Error(x),
        }
    }
}

//#endregion
