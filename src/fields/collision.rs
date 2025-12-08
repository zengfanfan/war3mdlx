use crate::*;

#[derive(Dbg, Default)]
pub struct CollisionShape {
    pub base: Node,
    pub shape: CollisionType,
    #[dbg(formatter = "fmtx")]
    pub vertices: Vec<Vec3>,
    pub bounds_radius: f32,
}

impl CollisionShape {
    pub const ID: u32 = MdlxMagic::CLID as u32;
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.base = Node::read_mdx(cur)?;
        this.shape = CollisionType::from(cur.readx()?);

        match this.shape {
            CollisionType::Box => this.vertices = cur.read_array(2)?,
            CollisionType::Plane => this.vertices = cur.read_array(1)?,
            CollisionType::Sphere => {
                this.vertices = cur.read_array(1)?;
                this.bounds_radius = cur.readx()?;
            },
            CollisionType::Cylinder => {
                this.vertices = cur.read_array(1)?;
                this.bounds_radius = cur.readx()?;
            },
            CollisionType::Error(v) => return ERR!("Unknown collision type: {}", v),
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];
        lines.append(&mut self.base.write_mdl(depth)?);
        lines.push(F!("{indent}{:?},", self.shape));
        MdlWriteType2!(lines, depth, "Vertices" => self.vertices);
        lines.pushx_if_n0(&F!("{indent}BoundsRadius"), &self.bounds_radius);
        return Ok(lines);
    }
}

#[derive(Debug, Default)]
pub enum CollisionType {
    #[default]
    Box,
    Plane,
    Sphere,
    Cylinder,
    Error(i32),
}
impl CollisionType {
    fn from(v: i32) -> Self {
        match v {
            0 => Self::Box,
            1 => Self::Plane,
            2 => Self::Sphere,
            3 => Self::Cylinder,
            _ => Self::Error(v),
        }
    }
}
