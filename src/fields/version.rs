use crate::*;

#[derive(Dbg, Default)]
pub struct Version {
    pub format_version: u32,
}

impl Version {
    pub const ID: u32 = MdlxMagic::VERS as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { format_version: cur.readx()? })
    }
}
