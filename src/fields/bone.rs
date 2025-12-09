use crate::*;

#[derive(Dbg, Default)]
pub struct Bone {
    pub base: Node,
    pub geoset_id: i32,
    pub geoanim_id: i32,
}

impl Bone {
    pub const ID: u32 = MdlxMagic::BONE as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { base: Node::read_mdx(cur)?, geoset_id: cur.readx()?, geoanim_id: cur.readx()? })
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.base = Node::read_mdl(block)?;
        this.geoset_id = -1;
        this.geoanim_id = -1;
        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "GeosetId" => this.geoset_id = f.value.to(),
                "GeosetAnimId" => this.geoanim_id = f.value.to(),
                _other => (),
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];
        lines.append(&mut self.base.write_mdl(depth)?);
        lines.push_if_nneg1(&F!("{indent}GeosetId"), &self.geoset_id);
        lines.push_if_nneg1(&F!("{indent}GeosetAnimId"), &self.geoanim_id);
        return Ok(lines);
    }
}
