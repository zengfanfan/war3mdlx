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

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let mut lines: Vec<String> = vec![];
        lines.append(&mut self.base.write_mdl(depth)?);
        lines.append(&mut self.track.write_mdl(depth)?);
        return Ok(lines);
    }
}

#[derive(Dbg, Default)]
pub struct EventTrack {
    pub global_seq_id: i32,
    pub frames: Vec<i32>,
}

impl EventTrack {
    pub const ID: u32 = MdlxMagic::KEVT as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        let n = cur.readx::<u32>()?;
        this.global_seq_id = cur.readx()?;
        for _ in 0..n {
            this.frames.push(cur.readx()?);
        }

        yesno!(this.global_seq_id == -1, Ok(this), ERR!("OMG! [EventObject.GlobalSequenceId] != -1 ?"))
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push_if_nneg1(&F!("{indent}GlobalSeqId"), &self.global_seq_id);
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
