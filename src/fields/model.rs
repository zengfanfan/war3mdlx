use crate::*;

#[derive(Dbg, SmartDefault)]
pub struct Model {
    pub name: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub min_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub max_extent: Vec3,
    #[default = 100]
    pub blend_time: u32,
}

impl Model {
    pub const ID: u32 = MdlxMagic::MODL;
    const NAME_SIZE: u32 = 336;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            name: cur.read_string(Self::NAME_SIZE)?,
            _unknown: cur.readx()?,
            bounds_radius: cur.readx()?,
            min_extent: cur.readx()?,
            max_extent: cur.readx()?,
            blend_time: cur.readx()?,
        })
    }

    pub fn write_mdx(&self, chunk: &mut MdxChunk) -> Result<(), MyError> {
        chunk.write_string(&self.name, Self::NAME_SIZE)?;
        chunk.write(&self._unknown)?;
        chunk.write(&self.bounds_radius)?;
        chunk.write(&self.min_extent)?;
        chunk.write(&self.max_extent)?;
        chunk.write(&self.blend_time)?;
        return Ok(());
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Build! { name: block.name.clone() };
        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "BoundsRadius" => this.bounds_radius = f.value.to()?,
                "MinimumExtent" => this.min_extent = f.value.to()?,
                "MaximumExtent" => this.max_extent = f.value.to()?,
                "BlendTime" => this.blend_time = f.value.to()?,
                _other => (), // ignore
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth + 1);
        let mut lines: Vec<String> = vec![];
        lines.push(F!("Model \"{}\" {{", self.name.escape()));
        if !(self.bounds_radius.is0() && self.min_extent.is0() && self.max_extent.is0()) {
            lines.pushx(&F!("{indent}BoundsRadius"), &self.bounds_radius);
            lines.pushx(&F!("{indent}MinimumExtent"), &self.min_extent);
            lines.pushx(&F!("{indent}MaximumExtent"), &self.max_extent);
        }
        lines.push(F!("{indent}BlendTime {},", self.blend_time));
        lines.push(F!("}}"));
        return Ok(lines);
    }
}
