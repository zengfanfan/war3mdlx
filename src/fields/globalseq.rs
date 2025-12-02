use crate::*;

#[derive(Dbg, Default)]
pub struct GlobalSequence {
    pub duration: u32,
}

impl GlobalSequence {
    pub const ID: u32 = MdlxMagic::GLBS as u32;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.duration = cur.readx()?;
        return Ok(this);
    }
}
