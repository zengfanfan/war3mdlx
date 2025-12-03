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
}
