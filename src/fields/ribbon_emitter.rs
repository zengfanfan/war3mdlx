use crate::*;

#[derive(Dbg, Default)]
pub struct RibbonEmitter {
    pub base: Node,

    pub height_above: f32,
    pub height_below: f32,
    pub alpha: f32,
    #[dbg(formatter = "fmtx")]
    pub color: Vec3,
    pub lifespan: f32,
    #[dbg(skip)]
    pub _unknown: i32,
    pub emit_rate: i32,
    pub rows: i32,
    pub columns: i32,
    pub material_id: i32,
    pub gravity: f32,

    #[dbg(formatter = "fmtxx")]
    pub height_above_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub height_below_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub alpha_anim: Option<Animation<f32>>,
    #[dbg(formatter = "fmtxx")]
    pub color_anim: Option<Animation<Vec3>>,
    #[dbg(formatter = "fmtxx")]
    pub texslot_anim: Option<Animation<i32>>,
    #[dbg(formatter = "fmtxx")]
    pub visibility: Option<Animation<f32>>,
}

impl RibbonEmitter {
    pub const ID: u32 = MdlxMagic::RIBB as u32;
    const ID_V: u32 = MdlxMagic::KRVS as u32; /* Visibility */
    const ID_HA: u32 = MdlxMagic::KRHA as u32; /* Height Above */
    const ID_HB: u32 = MdlxMagic::KRHB as u32; /* Height Below */
    const ID_A: u32 = MdlxMagic::KRAL as u32; /* Alpha */
    const ID_C: u32 = MdlxMagic::KRCO as u32; /* Color */
    const ID_TS: u32 = MdlxMagic::KRTX as u32; /* TextureSlot */

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
        this.height_above = cur.readx()?;
        this.height_below = cur.readx()?;
        this.alpha = cur.readx()?;
        this.color = cur.readx()?;
        this.lifespan = cur.readx()?;
        this._unknown = cur.readx()?;
        this.emit_rate = cur.readx()?;
        this.rows = cur.readx()?;
        this.columns = cur.readx()?;
        this.material_id = cur.readx()?;
        this.gravity = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                Self::ID_V => this.visibility = Some(Animation::read_mdx(cur)?),
                Self::ID_HA => this.height_above_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_HB => this.height_below_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_A => this.alpha_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_C => this.color_anim = Some(Animation::read_mdx(cur)?),
                Self::ID_TS => this.texslot_anim = Some(Animation::read_mdx(cur)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.base = Node::read_mdl(block)?;
        this.base.flags.insert(NodeFlags::RibbonEmitter);

        this.alpha = 1.0;
        this.color = Vec3::ONE;
        this.material_id = -1;

        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "HeightAbove" => this.height_above = f.value.to(),
                "HeightBelow" => this.height_below = f.value.to(),
                "Alpha" => this.alpha = f.value.to(),
                "Color" => this.color = f.value.to(),
                "EmissionRate" => this.emit_rate = f.value.to(),
                "LifeSpan" => this.lifespan = f.value.to(),
                "Gravity" => this.gravity = f.value.to(),
                "Rows" => this.rows = f.value.to(),
                "Columns" => this.columns = f.value.to(),
                "MaterialID" => this.material_id = f.value.to(),
                _other => (),
            );
        }

        for b in &block.blocks {
            match_istr!(b.typ.as_str(),
                "HeightAbove" => this.height_above_anim = Some(Animation::read_mdl(b)?),
                "HeightBelow" => this.height_below_anim = Some(Animation::read_mdl(b)?),
                "Alpha" => this.alpha_anim = Some(Animation::read_mdl(b)?),
                "Color" => this.color_anim = Some(Animation::read_mdl(b)?),
                "TextureSlot" => this.texslot_anim = Some(Animation::read_mdl(b)?),
                "Visibility" => this.visibility = Some(Animation::read_mdl(b)?),
                _other => (),
            );
        }

        this.color = this.color.reverse();
        this.color_anim = this.color_anim.map(|a| a.convert(|v| v.reverse()));

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];

        lines.append(&mut self.base.write_mdl(depth)?);

        let bgr = self.color.reverse();
        let bgr_anim = self.color_anim.as_ref().and_then(|a| Some(a.convert(|v| v.reverse())));

        lines.pushx_if_n0(&F!("{indent}EmissionRate"), &self.emit_rate);
        lines.pushx_if_n0(&F!("{indent}LifeSpan"), &self.lifespan);
        lines.pushx_if_n0(&F!("{indent}Gravity"), &self.gravity);
        lines.pushx_if_n0(&F!("{indent}Rows"), &self.rows);
        lines.pushx_if_n0(&F!("{indent}Columns"), &self.columns);
        lines.pushx_if_nneg1(&F!("{indent}MaterialID"), &self.material_id);

        MdlWriteAnimBoth!(lines, depth,
            "HeightAbove" => self.height_above_anim => 0.0 => self.height_above,
            "HeightBelow" => self.height_below_anim => 0.0 => self.height_below,
            "Alpha" => self.alpha_anim => 1.0 => self.alpha,
            "Color" => bgr_anim => Vec3::ONE => bgr,
        );
        MdlWriteAnim!(lines, depth,
            "TextureSlot" => self.texslot_anim,
            "Visibility" => self.visibility,
        );

        return Ok(lines);
    }
}
