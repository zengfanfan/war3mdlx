use crate::*;

#[derive(Dbg, Default)]
pub struct EventObject {
    pub base: Node,
    pub track: EventTrack,
}

impl EventObject {
    pub const ID: u32 = MdlxMagic::EVTS as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
        if cur.left() >= 8 {
            match cur.read_be()? {
                EventTrack::ID => this.track = EventTrack::read_mdx(cur)?,
                id => return ERR!("Unknown chunk in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.base = Node::read_mdl(block)?;
        for b in &block.blocks {
            match_istr!(b.typ.as_str(),
                "EventTrack" => this.track = EventTrack::read_mdl(b)?,
                _other => (),
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let mut lines: Vec<String> = vec![];
        lines.append(&mut self.base.write_mdl(depth)?);
        lines.append(&mut self.track.write_mdl(depth)?);
        return Ok(lines);
    }
}

#[derive(Dbg, Default)]
pub struct EventTrack {
    #[dbg(skip)]
    pub _unknown: i32,
    pub frames: Vec<i32>,
}

impl EventTrack {
    pub const ID: u32 = MdlxMagic::KEVT as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        let n = cur.readx::<u32>()?;
        this._unknown = cur.readx()?;
        for _ in 0..n {
            this.frames.push(cur.readx()?);
        }

        return Ok(this);
    }

    pub fn read_mdl(block: &MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        for f in &block.fields {
            if let MdlValue::Integer(i) = f.value {
                this.frames.push(i);
            }
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        if !self.frames.is_empty() {
            lines.push(F!("{indent}EventTrack {} {{", self.frames.len()));
            for f in &self.frames {
                lines.push(F!("{indent2}{},", f));
            }
            lines.push(F!("{indent}}}"));
        }
        return Ok(lines);
    }
}
