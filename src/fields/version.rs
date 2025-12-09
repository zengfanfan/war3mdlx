use crate::*;

#[derive(Dbg, Default)]
pub struct Version {
    pub format_version: i32,
}

impl Version {
    pub const ID: u32 = MdlxMagic::VERS as u32;
    pub const SUPPORTED_VERSION: [i32; 1] = [800];

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { format_version: cur.readx()? })
    }

    pub fn read_mdl(block: MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        for f in block.fields {
            if f.name.eq_icase("FormatVersion") {
                this.format_version = f.value.into();
                break; // only 1 field
            }
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        Ok(vec![F!("Version {{\n{}FormatVersion {},\n}}", indent!(depth + 1), self.format_version)])
    }
}
