use crate::*;

#[derive(Dbg, Default)]
pub struct Sequence {
    pub name: String,
    pub start_frame: i32,
    pub end_frame: i32,
    pub move_speed: f32,
    pub looping: bool,
    pub rarity: f32,
    #[dbg(skip)]
    pub _unknown: i32,
    pub bounds_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub min_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub max_extent: Vec3,
}

impl Sequence {
    pub const ID: u32 = MdlxMagic::SEQS as u32;
    const NAME_SIZE: u32 = 80;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(Self {
            name: cur.read_string(Self::NAME_SIZE)?,
            start_frame: cur.readx()?,
            end_frame: cur.readx()?,
            move_speed: cur.readx()?,
            looping: cur.readx::<u32>()? == 0,
            rarity: cur.readx()?,
            _unknown: cur.readx()?,
            bounds_radius: cur.readx()?,
            min_extent: cur.readx()?,
            max_extent: cur.readx()?,
        })
    }

    pub fn read_mdl(block: MdlBlock) -> Result<Self, MyError> {
        let mut this = Self::default();
        this.name = block.name;
        this.looping = true;
        for f in block.fields {
            match_istr!(f.name.as_str(),
                "MoveSpeed" => this.move_speed = f.value.into(),
                "NonLooping" => this.looping = false,
                "Rarity" => this.rarity = f.value.into(),
                "BoundsRadius" => this.bounds_radius = f.value.into(),
                "MinimumExtent" => this.min_extent = f.value.into(),
                "MaximumExtent" => this.max_extent = f.value.into(),
                "Interval" => {
                    let interval: Vec<i32> = f.value.into();
                    this.start_frame = interval.get(0).cloned().unwrap_or(0);
                    this.end_frame = interval.get(1).cloned().unwrap_or(0);
                },
                _other => (),
            );
        }
        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2) = (indent!(depth), indent!(depth + 1));
        let mut lines: Vec<String> = vec![];
        lines.push(F!("{indent}Anim \"{}\" {{", self.name));
        lines.push(F!("{indent2}Interval {{ {}, {} }},", self.start_frame, self.end_frame));
        lines.pushx_if_n0(&F!("{indent2}MoveSpeed"), &self.move_speed);
        yes!(!self.looping, lines.push(F!("{indent2}NonLooping,")));
        lines.pushx_if_n0(&F!("{indent2}Rarity"), &self.rarity);
        if !(self.bounds_radius.is0() && self.min_extent.is0() && self.max_extent.is0()) {
            lines.pushx(&F!("{indent2}BoundsRadius"), &self.bounds_radius);
            lines.pushx(&F!("{indent2}MinimumExtent"), &self.min_extent);
            lines.pushx(&F!("{indent2}MaximumExtent"), &self.max_extent);
        }
        lines.push(F!("{indent}}}"));
        return Ok(lines);
    }
}
