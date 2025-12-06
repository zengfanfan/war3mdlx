use crate::*;

pub struct MdxChunk {
    pub id: u32,
    pub body: Vec<u8>,
}

impl MdxChunk {
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        yes!(left = cur.left(), left < 4, EXIT!("reading chunk id: {}B left (need 4)", left));
        let id = cur.read_be()?;
        let estr = F!("reading chunk 0x{id:08X}({})", u32_to_ascii(id));

        yes!(left = cur.left(), left < 4, EXIT!("{} size: {}B left (need 4)", estr, left));
        let sz = cur.readx().or_else(|e| ERR!("{} size: {}", estr, e))?;
        vvlog!("chunk = 0x{:08X} ({}) [{}]", id, u32_to_ascii(id), sz);

        yes!(left = cur.left(), left < sz, EXIT!("{} body: {}B left (need {})", estr, left, sz));
        let body = cur.read_bytes(sz).or_else(|e| ERR!("{} body({}B): {}", estr, sz, e))?;
        vvvlog!("{}", pretty_hex(&body).replace("\n", "\n\t"));

        return Ok(MdxChunk { id, body });
    }
}
