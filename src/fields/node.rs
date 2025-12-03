use crate::*;

#[derive(Dbg, Default)]
pub struct Node {
    pub name: String,
    pub object_id: i32,
    pub parent_id: i32,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: u32, // see NodeFlags
    pub translation: Option<Animation<Vec3>>,
    pub rotation: Option<Animation<Vec4>>,
    pub scaling: Option<Animation<Vec3>>,
}

impl Node {
    const NAME_SIZE: u32 = 80;
    const ID_T: u32 = MdlxMagic::KGTR as u32; /* Translation */
    const ID_R: u32 = MdlxMagic::KGRT as u32; /* Rotation */
    const ID_S: u32 = MdlxMagic::KGSC as u32; /* Scaling */

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        let sz = cur.readx::<u32>()?;
        let body = cur.read_bytes(sz - 4)?;
        let mut cur = Cursor::new(&body); // use a new cursor

        this.name = cur.read_string(Self::NAME_SIZE)?;
        this.object_id = cur.readx()?;
        this.parent_id = cur.readx()?;
        this.flags = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_T => this.translation = Some(Animation::read_mdx(&mut cur, id)?),
                id @ Self::ID_R => this.rotation = Some(Animation::read_mdx(&mut cur, id)?),
                id @ Self::ID_S => this.scaling = Some(Animation::read_mdx(&mut cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}

pub enum NodeFlags {
    Helper = 0,
    DontInherit_Translation = 1 << 0,
    DontInherit_Rotation = 1 << 1,
    DontInherit_Scaling = 1 << 2,
    Billboarded = 1 << 3,
    Billboarded_Lock_X = 1 << 4,
    Billboarded_Lock_Y = 1 << 5,
    Billboarded_Lock_Z = 1 << 6,
    Camera_Anchored = 1 << 7,
    Bone = 1 << 8,
    Light = 1 << 9,
    EventObject = 1 << 10,
    Attachment = 1 << 11,
    ParticleEmitter = 1 << 12,
    CollisionShape = 1 << 13,
    RibbonEmitter = 1 << 14,
    PE2_Unshaded___PE1_UsesMdl = 1 << 15, // ParticleEmitter2.Unshaded / ParticleEmitter.EmitterUsesMdl
    PE2_SortPrimFarZ___PE1_UsesTga = 1 << 16, // ParticleEmitter2.SortPrimitivesFarZ / ParticleEmitter.EmitterUsesTga
    LineEmitter = 1 << 17,
    Unfogged = 1 << 18,
    ModelSpace = 1 << 19,
    XYQuad = 1 << 20,
}
