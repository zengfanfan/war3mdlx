use crate::*;

//#region Geoset

#[derive(Dbg, Default)]
pub struct Geoset {
    #[dbg(formatter = "fmtx")]
    pub vertices: Vec<Vec3>,
    #[dbg(formatter = "fmtx")]
    pub normals: Vec<Vec3>,
    #[dbg(formatter = "fmtx")]
    pub uvs: Vec<Vec2>,
    #[dbg(formatter = "fmtx")]
    pub triangles: Vec<i16>,
    #[dbg(formatter = "fmtx")]
    pub vtxgrps: Vec<u8>,
    #[dbg(formatter = "fmtx")]
    pub mtxgrpcnts: Vec<u32>,
    #[dbg(formatter = "fmtx")]
    pub mtx_indices: Vec<u32>,
    pub material_id: u32,
    pub sel_group: u32,
    pub sel_type: u32,
    #[dbg(formatter = "fmtx")]
    pub bound_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub min_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub max_extent: Vec3,
    pub anim_extents: Vec<AnimExtent>,
}
#[derive(Dbg, Default)]
pub struct AnimExtent {
    #[dbg(formatter = "fmtx")]
    pub bound_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub min_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub max_extent: Vec3,
}

impl Geoset {
    pub const ID: u32 = MdlxMagic::GEOS as u32;
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        //[test] check if some chunk canbe more than 1 (python check all MDLs and MDXs)

        while cur.left() > 16 {
            let id = cur.read_be()?;
            let n = cur.readx()?;
            if id == MdlxMagic::VRTX as u32 {
                this.vertices = cur.read_array(n)?;
            } else if id == MdlxMagic::NRMS as u32 {
                this.normals = cur.read_array(n)?;
            } else if id == MdlxMagic::PTYP as u32 {
                // face types: [test] check if this can be more than 1
                // face types: [test] check if there are value other than 4
                let face_types: Vec<u32> = cur.read_array(n)?;
                if face_types.iter().any(|&x| x != 4) {
                    return ERR!("Only triangle(4) is supported: {:?}", face_types);
                }
            } else if id == MdlxMagic::PCNT as u32 {
                // face type count: [test] check if this can be more than 1
                let v: Vec<u32> = cur.read_array(n)?;
            } else if id == MdlxMagic::PVTX as u32 {
                // face type count: [test] check if this can be more than 1
                this.triangles = cur.read_array(n)?;
            } else if id == MdlxMagic::GNDX as u32 {
                this.vtxgrps = cur.read_array(n)?;
            } else if id == MdlxMagic::MTGC as u32 {
                this.mtxgrpcnts = cur.read_array(n)?;
            } else if id == MdlxMagic::MATS as u32 {
                this.mtx_indices = cur.read_array(n)?;
            } else if id == MdlxMagic::UVAS as u32 {
            } else if id == MdlxMagic::UVBS as u32 {
                this.uvs = cur.read_array(n)?;
            } else {
                this.material_id = id;
                this.sel_group = n;
                this.sel_type = cur.readx()?;
                this.bound_radius = cur.readx()?;
                this.min_extent = cur.readx()?;
                this.max_extent = cur.readx()?;
                let en = cur.readx()?;
                for _ in 0..en {
                    this.anim_extents.push(AnimExtent {
                        bound_radius: cur.readx()?,
                        min_extent: cur.readx()?,
                        max_extent: cur.readx()?,
                    });
                }
            }
        }

        return Ok(this);
    }
}

//#endregion
//#region
//#endregion
//#region
//#endregion
//#region
//#endregion
//#region GeosetAnim

#[derive(Dbg, Default)]
pub struct GeosetAnim {
    pub alpha: f32,
    pub flags: u32,
    pub color: Vec3,
    pub geoset_id: i32,
    pub alpha_anim: Option<Animation<f32>>,
    pub color_anim: Option<Animation<Vec3>>,
}

impl GeosetAnim {
    pub const ID: u32 = MdlxMagic::GEOA as u32;
    pub const ID_ALPHA: u32 = MdlxMagic::KGAO as u32;
    pub const ID_COLOR: u32 = MdlxMagic::KGAC as u32;

    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.alpha = cur.readx()?;
        this.flags = cur.readx()?;
        this.color = cur.readx()?;
        this.geoset_id = cur.readx()?;

        while cur.left() > 16 {
            match cur.read_be()? {
                id @ Self::ID_ALPHA => this.alpha_anim = Some(Animation::parse_mdx(cur, id)?),
                id @ Self::ID_COLOR => this.color_anim = Some(Animation::parse_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }
}

//#endregion
