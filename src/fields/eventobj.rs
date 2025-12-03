use crate::*;

#[derive(Dbg, Default)]
pub struct EventObject {
    pub base: Node,
    pub track: EventTrack,
}

impl EventObject {
    pub const ID: u32 = MdlxMagic::EVTS as u32;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::parse_mdx(cur)?;
        if cur.left() >= 8 {
            match cur.read_be()? {
                EventTrack::ID => this.track = EventTrack::parse_mdx(cur)?,
                id => return ERR!("Unknown chunk in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}

#[derive(Dbg, Default)]
pub struct EventTrack {
    pub global_seq_id: i32,
    pub frames: Vec<i32>,
}

impl EventTrack {
    pub const ID: u32 = MdlxMagic::KEVT as u32;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        let n = cur.readx::<u32>()?;
        this.global_seq_id = cur.readx()?;
        for _ in 0..n {
            this.frames.push(cur.readx()?);
        }

        yesno!(this.global_seq_id == -1, Ok(this), ERR!("OMG! [EventObject.GlobalSequenceId] != -1 ?"))
    }
}
