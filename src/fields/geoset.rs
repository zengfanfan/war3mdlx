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
    pub face_types: Vec<FaceType>,
    #[dbg(formatter = "fmtx")]
    pub face_vtxcnts: Vec<i32>,
    #[dbg(formatter = "fmtx")]
    pub face_vertices: Vec<i16>,
    #[dbg(formatter = "fmtx")]
    pub vtxgrps: Vec<u8>,
    #[dbg(formatter = "fmtx")]
    pub mtxgrpcnts: Vec<u32>,
    #[dbg(formatter = "fmtx")]
    pub mtx_indices: Vec<u32>,
    pub material_id: i32,
    pub sel_group: u32,
    pub sel_type: u32, // 0=None, 4=Unselectable
    pub extent: BoundExtent,
    pub anim_extents: Vec<BoundExtent>,
}

#[derive(Dbg, Default)]
pub struct BoundExtent {
    #[dbg(formatter = "fmtx")]
    pub bound_radius: f32,
    #[dbg(formatter = "fmtx")]
    pub min_extent: Vec3,
    #[dbg(formatter = "fmtx")]
    pub max_extent: Vec3,
}
impl BoundExtent {
    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(BoundExtent { bound_radius: cur.readx()?, min_extent: cur.readx()?, max_extent: cur.readx()? })
    }
    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        Ok(vec![
            F!("{}BoundsRadius {},", indent!(depth), fmtx(&self.bound_radius)),
            F!("{}MinimumExtent {},", indent!(depth), fmtx(&self.min_extent)),
            F!("{}MaximumExtent {},", indent!(depth), fmtx(&self.max_extent)),
        ])
    }
}

#[repr(u32)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FaceType {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    #[default]
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
    Quads = 7,
    QuadStrip = 8,
    Polygons = 9,
    Error(u32),
}
impl FaceType {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::Points,
            1 => Self::Lines,
            2 => Self::LineLoop,
            3 => Self::LineStrip,
            4 => Self::Triangles,
            5 => Self::TriangleStrip,
            6 => Self::TriangleFan,
            7 => Self::Quads,
            8 => Self::QuadStrip,
            9 => Self::Polygons,
            x => Self::Error(x),
        }
    }
}

impl Geoset {
    pub const ID: u32 = MdlxMagic::GEOS as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        while cur.left() >= 16 {
            let (id, n) = (cur.read_be::<u32>()?, cur.readx::<u32>()?);
            if id == MdlxMagic::VRTX as u32 {
                this.vertices = cur.read_array(n)?;
            } else if id == MdlxMagic::NRMS as u32 {
                this.normals = cur.read_array(n)?;
            } else if id == MdlxMagic::PTYP as u32 {
                yes!(n > 1, EXIT!("OMG! [face type count] {n} > 1 ?"));
                this.face_types = cur.read_array::<u32>(n)?.iter().map(|a| FaceType::from(*a)).collect();
                if this.face_types.iter().any(|&x| x != FaceType::Triangles) {
                    EXIT!("Only triangle(4) is supported: {:?}", this.face_types);
                }
            } else if id == MdlxMagic::PCNT as u32 {
                yes!(n > 1, EXIT!("OMG! [vertex count for each face type] {n} > 1 ?"));
                this.face_vtxcnts = cur.read_array(n)?;
            } else if id == MdlxMagic::PVTX as u32 {
                this.face_vertices = cur.read_array(n)?;
            } else if id == MdlxMagic::GNDX as u32 {
                this.vtxgrps = cur.read_array(n)?;
            } else if id == MdlxMagic::MTGC as u32 {
                this.mtxgrpcnts = cur.read_array(n)?;
            } else if id == MdlxMagic::MATS as u32 {
                this.mtx_indices = cur.read_array(n)?;
            } else if id == MdlxMagic::UVAS as u32 {
                yes!(n > 1, EXIT!("OMG! [number for UV group] {n} > 1 ?"));
            } else if id == MdlxMagic::UVBS as u32 {
                this.uvs = cur.read_array(n)?;
            } else {
                this.material_id = id.swap_bytes() as i32;
                this.sel_group = n;
                this.sel_type = cur.readx()?;
                this.extent = BoundExtent::read_mdx(cur)?;
                let en = cur.readx()?;
                for _ in 0..en {
                    this.anim_extents.push(BoundExtent::read_mdx(cur)?);
                }
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let (indent, indent2, indent3) = (indent!(depth), indent!(depth + 1), indent!(depth + 2));
        let mut lines: Vec<String> = vec![];

        MdlWriteType2!(lines, depth, "Vertices" => self.vertices);
        MdlWriteType2!(lines, depth, "Normals" => self.normals);
        MdlWriteType2!(lines, depth, "TVertices" => self.uvs);
        {
            lines.push(F!("{indent}VertexGroup {{"));
            lines.append(&mut self.vtxgrps.iter().map(|x| F!("{indent2}{},", fmtx(x))).collect::<Vec<String>>());
            lines.push(F!("{indent}}}"));
        }
        {
            lines.push(F!("{indent}Faces {} {} {{", self.face_types.len(), self.face_vertices.len()));
            let mut i = 0_usize;
            for (t, n) in self.face_types.iter().zip(self.face_vtxcnts.iter()) {
                let n = *n as usize;
                lines.push(F!("{indent2}{:?} {{", t));
                if let Some(slice) = &self.face_vertices.get(i..i + n) {
                    let s = slice.iter().map(|x| fmtx(x)).collect::<Vec<String>>().join(", ");
                    lines.push(F!("{indent3}{{ {} }},", s));
                }
                lines.push(F!("{indent2}}}"));
                i += n;
            }
            lines.push(F!("{indent}}}"));
        }
        {
            lines.push(F!("{indent}Groups {} {} {{", self.mtxgrpcnts.len(), self.mtx_indices.len()));
            let mut i = 0_usize;
            for n in &self.mtxgrpcnts {
                let n = *n as usize;
                let mut slist: Vec<String> = vec![];
                slist.push(F!("Matrices {{"));
                if let Some(slice) = &self.mtx_indices.get(i..i + n) {
                    let s = slice.iter().map(|x| fmtx(x)).collect::<Vec<String>>().join(", ");
                    slist.push(F!("{}", s));
                }
                slist.push(F!("}},"));
                lines.push(F!("{indent2}{}", slist.join(" ")));
                i += n;
            }
            lines.push(F!("{indent}}}"));
        }

        lines.append(&mut self.extent.write_mdl(depth)?);
        for en in &self.anim_extents {
            lines.push(F!("{indent}Anim {{"));
            lines.append(&mut en.write_mdl(depth + 1)?);
            lines.push(F!("{indent}}}"));
        }

        lines.push_if_nneg1(&F!("{indent}MaterialID"), &self.material_id);
        lines.push_if_nneg1(&F!("{indent}SelectionGroup"), &self.sel_group);
        yes!(self.sel_type != 0, lines.push(F!("{indent}Unselectable,")));

        return Ok(lines);
    }
}

//#endregion
//#region GeosetAnim

#[derive(Dbg, Default)]
pub struct GeosetAnim {
    pub alpha: f32,
    pub flags: GeosetAnimFlags,
    #[dbg(formatter = "fmtx")]
    pub color: Vec3,
    pub geoset_id: i32,
    pub alpha_anim: Option<Animation<f32>>,
    pub color_anim: Option<Animation<Vec3>>,
}
bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GeosetAnimFlags : u32 {
        const DropShadow = 1 << 0;
        const UseColor = 1 << 1;
    }
}

impl GeosetAnim {
    pub const ID: u32 = MdlxMagic::GEOA as u32;
    const ID_ALPHA: u32 = MdlxMagic::KGAO as u32;
    const ID_COLOR: u32 = MdlxMagic::KGAC as u32;

    pub fn read_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let mut this = Self::default();

        this.alpha = cur.readx()?;
        this.flags = GeosetAnimFlags::from_bits_retain(cur.readx()?);
        this.color = cur.readx()?;
        this.geoset_id = cur.readx()?;

        while cur.left() >= 16 {
            match cur.read_be()? {
                id @ Self::ID_ALPHA => this.alpha_anim = Some(Animation::read_mdx(cur, id)?),
                id @ Self::ID_COLOR => this.color_anim = Some(Animation::read_mdx(cur, id)?),
                id => return ERR!("Unknown animation in {}: {} (0x{:08X})", TNAME!(), u32_to_ascii(id), id),
            }
        }

        return Ok(this);
    }

    pub fn write_mdl(&self, depth: u8) -> Result<Vec<String>, MyError> {
        let indent = indent!(depth);
        let mut lines: Vec<String> = vec![];
        lines.push_if_nneg1(&F!("{indent}GeosetId"), &self.geoset_id);
        yes!(self.flags.contains(GeosetAnimFlags::DropShadow), lines.push(F!("{indent}DropShadow,")));

        MdlWriteAnimBoth!(lines, depth, "Alpha" => self.alpha_anim => 1.0 => self.alpha);
        // [todo] check if color anim need reverse as static does
        if self.flags.contains(GeosetAnimFlags::UseColor) {
            if let Some(anim) = &self.color_anim {
                let bgr = anim.convert(|v| v.reverse());
                _MdlWriteAnim!(lines, depth, "Color" => bgr);
            } else if self.color != Vec3::ONE {
                let bgr = self.color.reverse();
                _MdlWriteAnimStatic!(lines, depth, "Color" => bgr);
            }
        }

        return Ok(lines);
    }
}

//#endregion
