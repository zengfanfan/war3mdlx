use crate::*;

#[derive(Dbg, Default)]
pub struct Attachment {
    pub base: Node,
    pub path: String,
    #[dbg(skip)]
    pub _unknown: i32,
    pub attachment_id: i32,
    pub visibility: Option<Animation<f32>>,
}

impl Attachment {
    pub const ID: u32 = MdlxMagic::ATCH as u32;
    const ID_V: u32 = MdlxMagic::KATV as u32; /* Visibility */
    const PATH_SIZE: u32 = 256;
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
        this.path = cur.read_string(Self::PATH_SIZE)?;
        this._unknown = cur.readx()?;
        this.attachment_id = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_V => this.visibility = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}
