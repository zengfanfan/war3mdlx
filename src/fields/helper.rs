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
}
