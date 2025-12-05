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

#[repr(u32)]
#[derive(Debug)]
pub enum CollisionType {
    Box = 0,
    Plane = 1,
    Sphere = 2,
    Cylinder = 3,
    Error(u32),
}
impl Default for CollisionType {
    fn default() -> Self {
        CollisionType::Box
    }
}
impl CollisionType {
    fn from(v: u32) -> CollisionType {
        match v {
            0 => CollisionType::Box,
            1 => CollisionType::Plane,
            2 => CollisionType::Sphere,
            3 => CollisionType::Cylinder,
            _ => CollisionType::Error(v),
        }
    }
}
