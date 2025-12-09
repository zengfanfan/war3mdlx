use crate::*;

#[derive(Dbg, Default)]
pub struct Helper {
    pub base: Node,
}

impl Helper {
    pub const ID: u32 = MdlxMagic::HELP as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self { base: Node::read_mdx(cur)? })
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        Ok(Self { base: Node::read_mdl(block)? })
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        self.base.write_mdl(depth)
    }
}
