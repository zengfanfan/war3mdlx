use crate::*;

#[derive(Dbg, Default)]
pub struct PivotPoint {
    pub position: Vec3,
}

impl PivotPoint {
    pub const ID: u32 = MdlxMagic::PIVT as u32;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.position = cur.readx()?;
        return Ok(this);
    }
}
