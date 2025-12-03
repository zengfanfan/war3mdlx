use crate::*;

#[derive(Dbg, Default)]
pub struct GlobalSequence {
    pub duration: u32,
}

impl GlobalSequence {
    pub const ID: u32 = MdlxMagic::GLBS as u32;
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.duration = cur.readx()?;
        return Ok(this);
    }
}

impl Formatter for Vec<GlobalSequence> {
    fn debug(&self) -> String {
        let s = self.iter().map(|x| fmtx(&x.duration)).collect::<Vec<_>>().join(", ");
        return format!("[{}]", s);
    }
}
