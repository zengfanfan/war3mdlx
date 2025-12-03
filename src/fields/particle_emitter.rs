use crate::*;

#[derive(Dbg, Default)]
pub struct ParticleEmitter {
    pub base: Node,
    pub emission_rate: f32,
    pub gravity: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub path: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub life_span: f32,
    pub speed: f32,
    pub visibility: Option<Animation<f32>>,
    pub emission_rate_anim: Option<Animation<f32>>,
    pub gravity_anim: Option<Animation<f32>>,
    pub longitude_anim: Option<Animation<f32>>,
    pub latitude_anim: Option<Animation<f32>>,
    pub life_span_anim: Option<Animation<f32>>,
    pub speed_anim: Option<Animation<f32>>,
}

impl ParticleEmitter {
    pub const ID: u32 = MdlxMagic::PREM as u32;
    const ID_V: u32 = MdlxMagic::KPEV as u32; /* Visibility */
    const ID_ER: u32 = MdlxMagic::KPEE as u32; /* Emission rate */
    const ID_G: u32 = MdlxMagic::KPEG as u32; /* Gravity */
    const ID_LO: u32 = MdlxMagic::KPLN as u32; /* Longitude */
    const ID_LA: u32 = MdlxMagic::KPLT as u32; /* Latitude */
    const ID_LS: u32 = MdlxMagic::KPEL as u32; /* Life span */
    const ID_SPD: u32 = MdlxMagic::KPES as u32; /* Speed */
    const NAME_SIZE: u32 = 256;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::parse_mdx(cur)?;
        this.emission_rate = cur.readx()?;
        this.gravity = cur.readx()?;
        this.longitude = cur.readx()?;
        this.latitude = cur.readx()?;
        this.path = cur.read_string(Self::NAME_SIZE)?;
        this._unknown = cur.readx()?;
        this.life_span = cur.readx()?;
        this.speed = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_V => this.visibility = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_ER => this.emission_rate_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_G => this.gravity_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_LO => this.longitude_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_LA => this.latitude_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_LS => this.life_span_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_SPD => this.speed_anim = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}
