use crate::*;

#[derive(Dbg, Default)]
pub struct ParticleEmitter {
    pub base: Node,

    pub emit_rate: f32,
    pub gravity: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub path: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub lifespan: f32,
    pub speed: f32,

    pub visibility: Option<Animation<f32>>,
    pub emit_rate_anim: Option<Animation<f32>>,
    pub gravity_anim: Option<Animation<f32>>,
    pub longitude_anim: Option<Animation<f32>>,
    pub latitude_anim: Option<Animation<f32>>,
    pub lifespan_anim: Option<Animation<f32>>,
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
    const PATH_SIZE: u32 = 256;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
        this.emit_rate = cur.readx()?;
        this.gravity = cur.readx()?;
        this.longitude = cur.readx()?;
        this.latitude = cur.readx()?;
        this.path = cur.read_string(Self::PATH_SIZE)?;
        this._unknown = cur.readx()?;
        this.lifespan = cur.readx()?;
        this.speed = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_V => this.visibility = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_ER => this.emit_rate_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_G => this.gravity_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_LO => this.longitude_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_LA => this.latitude_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_LS => this.lifespan_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_SPD => this.speed_anim = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];

        lines.append(&mut self.base.write_mdl(depth)?);
        let flags = self.base.flags;
        if flags.contains(NodeFlags::ParticleEmitter) {
            yes!(flags.contains(NodeFlags::PE1UsesMdl), lines.push(F!("{indent}EmitterUsesMDL,")));
            yes!(flags.contains(NodeFlags::PE1UsesTga), lines.push(F!("{indent}EmitterUsesTGA,")));
        }

        MdlWriteAnimStatic!(lines, depth,
            "EmissionRate" => self.emit_rate_anim => 0.0 => self.emit_rate,
            "Gravity" => self.gravity_anim => 0.0 => self.gravity,
            "Longitude" => self.longitude_anim => 0.0 => self.longitude,
            "Latitude" => self.latitude_anim => 0.0 => self.latitude,
        );
        MdlWriteAnim!(lines, depth,
            "EmissionRate" => self.emit_rate_anim,
            "Gravity" => self.gravity_anim,
            "Longitude" => self.longitude_anim,
            "Latitude" => self.latitude_anim,
            "Visibility" => self.visibility,
        );

        {
            let mut tines: Vec<String> = vec![];
            MdlWriteAnimBoth!(tines, depth + 1,
                "LifeSpan" => self.lifespan_anim => 0.0 => self.lifespan,
                "InitVelocity" => self.speed_anim => 0.0 => self.speed,
            );
            tines.pushx_if_n0(&F!("{indent2}Path"), &self.path);
            if !tines.is_empty() {
                lines.push(F!("{indent}Particle {{"));
                lines.append(&mut tines);
                lines.push(F!("{indent}}}"));
            }
        }

        return Ok(lines);
    }
}
