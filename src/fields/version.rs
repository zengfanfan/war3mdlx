use crate::*;

#[derive(Dbg, Default)]
pub struct Version {
    pub format_version: u32,
}

impl Version {
    pub const ID: u32 = MdlxMagic::VERS as u32;
    pub const SUPPORTED_VERSION: [u32; 1] = [800];

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { format_version: cur.readx()? })
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        Ok(vec![F!("Version {{\n{}FormatVersion {},\n}}", indent!(depth + 1), self.format_version)])
    }
}
