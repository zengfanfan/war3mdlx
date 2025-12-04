use crate::*;

#[derive(Dbg, Default)]
pub struct Node {
    pub name: String,
    pub object_id: i32,
    pub parent_id: i32,
    #[dbg(fmt = "0x{:08X}")]
    pub flags: NodeFlags, // see NodeFlags
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
        this.flags = NodeFlags::from_bits_retain(cur.readx()?);

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

//#region HeadOrTail

bitflags! {
    #[derive(Default)]
    pub struct NodeFlags: u32 {
        const All = !0;
        const Helper = 0;
        const DontInheritTranslation = 1 << 0;
        const DontInheritRotation = 1 << 1;
        const DontInheritScaling = 1 << 2;
        const Billboarded = 1 << 3;
        const BillboardedLockX = 1 << 4;
        const BillboardedLockY = 1 << 5;
        const BillboardedLockZ = 1 << 6;
        const CameraAnchored = 1 << 7;
        const Bone = 1 << 8;
        const Light = 1 << 9;
        const EventObject = 1 << 10;
        const Attachment = 1 << 11;
        const ParticleEmitter = 1 << 12;
        const CollisionShape = 1 << 13;
        const RibbonEmitter = 1 << 14;
        const PE2Unshaded = 1 << 15;     // ParticleEmitter2.Unshaded
        const PE1UsesMdl = 1 << 15;      // ParticleEmitter.EmitterUsesMdl
        const PE2SortPrimFarZ = 1 << 16; // ParticleEmitter2.SortPrimitivesFarZ
        const PE1UsesTga = 1 << 16;      // ParticleEmitter.EmitterUsesTga
        const LineEmitter = 1 << 17;
        const Unfogged = 1 << 18;
        const ModelSpace = 1 << 19;
        const XYQuad = 1 << 20;
    }
}

//#endregion
