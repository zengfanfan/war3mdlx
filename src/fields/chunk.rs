use crate::*;

pub struct MdxChunk {
    pub id: u32,
    pub body: Vec<u8>,
}

impl MdxChunk {
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let id = cur.read_be()?;
        let sz = cur.readx()?;
        vvlog!("chunk = 0x{:08X} ({}) [{}]", id, u32_to_ascii(id), sz);
        let body = cur.read_bytes(sz)?;
        vvvlog!("{}", pretty_hex(&body).replace("\n", "\n\t"));
        return Ok(MdxChunk { id, body });
    }
}
