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

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        Ok(vec![F!("{}Duration {},", indent!(depth), self.duration)])
    }
}

impl Formatter for Vec<GlobalSequence> {
    fn fmt(&self) -> String {
        let s = self.iter().map(|x| fmtx(&x.duration)).collect::<Vec<_>>().join(", ");
        return F!("[{}]", s);
    }
}
