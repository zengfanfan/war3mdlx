use crate::*;

#[derive(Dbg, Default)]
pub struct Version {
    pub format_version: i32,
}

impl Version {
    pub const ID: u32 = MdlxMagic::VERS;
    pub const SUPPORTED_VERSION: [i32; 1] = [800];

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { format_version: cur.readx()? })
    }

    pub fn write_mdx(&self, chunk: &mut MdxChunk) -> Result<(), MyError> {
        chunk.write(&self.format_version)
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Build!();
        for f in &block.fields {
            elog!(" * {:?}", f);
            match_istr!(f.name.as_str(),
                "FormatVersion" => this.format_version = f.value.to()?,
                _other => return f.unexpect(),
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        return Ok(vec![
            F!("{indent}Version {{"),
            F!("{indent2}FormatVersion {},", self.format_version),
            F!("{indent}}}"),
        ]);
    }
}
