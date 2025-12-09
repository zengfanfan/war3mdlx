use crate::*;

#[derive(Dbg, Default)]
pub struct Attachment {
    pub base: Node,

    pub path: String,
    #[dbg(skip)]
    pub _unknown: i32,
    #[dbg(fmt = "{:?}")]
    pub attachment_id: Option<i32>,
    pub aindex: i32, // the order appears in the file

    #[dbg(formatter = "fmtxx")]
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
        this.attachment_id = Some(cur.readx()?);

        while cur.left() >= 16 {
            match cur.read_be()? {
                Self::ID_V => this.visibility = Some(Animation::read_mdx(cur)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.base = Node::read_mdl(block)?;
        for f in &block.fields {
            match_istr!(f.name.as_str(),
                "Path" => this.path = f.value.to(),
                "AttachmentID" => this.attachment_id = Some(f.value.to()),
                _other => (),
            );
        }
        for b in &block.blocks {
            match_istr!(b.typ.as_str(),
                "Visibility" => this.visibility = Some(Animation::read_mdl(b)?),
                _other => (),
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];
        lines.append(&mut self.base.write_mdl(depth)?);
        if let Some(aid) = self.attachment_id {
            lines.push_if(aid != self.aindex, F!("{indent}AttachmentID {},", aid));
        }
        lines.pushx_if_n0(&F!("{indent}Path"), &self.path);
        MdlWriteAnim!(lines, depth, "Visibility" => self.visibility);
        return Ok(lines);
    }
}
