use crate::*;

pub fn cursor_read_bytes(cur: &mut Cursor<&Vec<u8>>, n: u32) -> Result<Vec<u8>, MyError> {
    let mut body = vec![0u8; n as usize];
    cur.read_exact(&mut body)?;
    return Ok(body);
}
