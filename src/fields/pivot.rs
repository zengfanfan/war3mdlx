use crate::*;

#[derive(Dbg, Default)]
pub struct PivotPoint {
    #[dbg(fmt = "{:?}")]
    pub position: Vec3,
}

impl PivotPoint {
    pub const ID: u32 = MdlxMagic::PIVT;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { position: cur.readx()? })
    }

    pub fn write_mdx(&self, chunk: &mut MdxChunk) -> Result<(), MyError> {
        chunk.write(&self.position)
    }

    pub fn read_mdl(field: &MdlField) -> Result<Self, MyError> {
        Ok(Self { position: field.value.to()? })
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        Ok(vec![F!("{}{},", indent!(depth), fmtx(&self.position))])
    }
}

impl Formatter for Vec<PivotPoint> {
    fn fmt(&self) -> String {
        let s = self.iter().map(|x| fmtx(&x.position)).collect::<Vec<_>>().join(", ");
        return F!("[{}]", s);
    }
}
