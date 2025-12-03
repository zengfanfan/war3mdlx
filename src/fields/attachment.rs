use crate::*;

#[derive(Dbg, Default)]
pub struct Attachment {
    pub base: Node,
    pub path: String,
    #[dbg(skip)]
    pub _unknown: u32,
    pub attachment_id: u32,
    pub visibility: Option<Animation<f32>>,
}

impl Attachment {
    pub const ID: u32 = MdlxMagic::ATCH as u32;
    pub const ID_V: u32 = MdlxMagic::KATV as u32;
    pub const NAME_SIZE: u32 = 256;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::parse_mdx(cur)?;
        this.path = cur.read_string(Self::NAME_SIZE)?;
        this._unknown = cur.readx()?;
        this.attachment_id = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_V => this.visibility = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}
