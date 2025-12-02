use crate::*;

#[derive(Debug)]
#[repr(u32)]
pub enum MdlxMagic {
    MDLX = 0x4D444C58,
    VERS = 0x56455253,
    MODL = 0x4D4F444C,
    ///* Sequences */
    SEQS = 0x53455153,
    ///* Global Sequences */
    GLBS = 0x474C4253,
    ///* Textures */
    TEXS = 0x54455853,
    ///* Materials */
    MTLS = 0x4D544C53,
    LAYS = 0x4C415953, /* Layers */
    KMTA = 0x4B4D5441, /* Layer: Alpha */
    KMTF = 0x4B4D5446, /* Layer: Texture ID */
    ///* Texture Animations */
    TXAN = 0x5458414E,
    KTAT = 0x4B544154, /* Translation */
    KTAR = 0x4B544152, /* Rotation */
    KTAS = 0x4B544153, /* Scaling */
    ///* Geosets */
    GEOS = 0x47454F53,
    VRTX = 0x56525458, /* Vertex Position */
    NRMS = 0x4E524D53, /* Vertex Normal */
    PTYP = 0x50545950, /* Face Type */
    PCNT = 0x50434E54, /* Vertices Group Count */
    PVTX = 0x50565458, /* Vertices of Faces / 3 */
    GNDX = 0x474E4458, /* Vertex Group */
    MTGC = 0x4D544743, /* Group Matrices */
    MATS = 0x4D415453, /* Group Matrices Indices */
    UVAS = 0x55564153, /* UVs */
    UVBS = 0x55564253, /* UVs */
    ///* Geoset Animations */
    GEOA = 0x47454F41,
    KGAO = 0x4B47414F, /* Alpha */
    KGAC = 0x4B474143, /* Color */
    ///* Pivot Points */
    PIVT = 0x50495654,
    ///* Cameras */
    CAMS = 0x43414D53,
    KCTR = 0x4B435452, /* Position Translation */
    KCRL = 0x4B43524C, /* Rotation */
    KTTR = 0x4B545452, /* Target Translation */
    ///* Node */
    KGTR = 0x4B475452, /* Translation */
    KGRT = 0x4B475254, /* Rotation */
    KGSC = 0x4B475343, /* Scaling */
    ///* Bones */
    BONE = 0x424F4E45,
    ///* Lights */
    LITE = 0x4C495445,
    KLAV = 0x4B4C4156, /* Visibility */
    KLAS = 0x4B4C4153, /* AttenuationStart */
    KLAE = 0x4B4C4145, /* AttenuationEnd */
    KLAC = 0x4B4C4143, /* Color */
    KLAI = 0x4B4C4149, /* Intensity */
    KLBC = 0x4B4C4243, /* Ambient Color */
    KLBI = 0x4B4C4249, /* Ambient Intensity */
    ///* Helpers */
    HELP = 0x48454C50,
    ///* Attachments */
    ATCH = 0x41544348,
    KATV = 0x4B415456, /* Attachment Visibility */
    ///* Event Objects */
    EVTS = 0x45565453,
    KEVT = 0x4B455654, /* Event Object Tracks */
    ///* Collision Shapes */
    CLID = 0x434C4944,
    ///* Particle Emitters */
    PREM = 0x5052454D,
    KPEV = 0x4B504556, /* Particle Emitter Visibility */
    KPEE = 0x4B504545, /* Particle Emitter EmissionRate */
    KPEG = 0x4B504547, /* Particle Emitter Gravity */
    KPLN = 0x4B504C4E, /* Particle Emitter Longitude */
    KPLT = 0x4B504C54, /* Particle Emitter Latitude */
    KPEL = 0x4B50454C, /* Particle Emitter LifeSpan */
    KPES = 0x4B504553, /* Particle Emitter Speed */
    ///* Particle Emitters 2 */
    PRE2 = 0x50524532,
    KP2V = 0x4B503256, /* Particle Emitter 2 Visibility */
    KP2E = 0x4B503245, /* Particle Emitter 2 Emission Rate */
    KP2W = 0x4B503257, /* Particle Emitter 2 Width */
    KP2N = 0x4B50324E, /* Particle Emitter 2 Length */
    KP2S = 0x4B503253, /* Particle Emitter 2 Speed */
    KP2L = 0x4B50324C, /* Particle Emitter 2 Latitude */
    KP2R = 0x4B503252, /* Particle Emitter 2 Variation */
    KP2G = 0x4B503247, /* Particle Emitter 2 Gravity */
    ///* Ribbon Emitters */
    RIBB = 0x52494242,
    KRVS = 0x4B525653, /* Ribbon Emitter Visibility */
    KRHA = 0x4B524841, /* Ribbon Emitter Height Above */
    KRHB = 0x4B524842, /* Ribbon Emitter Height Below */
    KRAL = 0x4B52414C, /* Ribbon Emitter Alpha */
    KRCO = 0x4B52434F, /* Ribbon Emitter Color */
    KRTX = 0x4B525458, /* Ribbon Emitter TextureSlot */
}

#[derive(Dbg, Default)]
pub struct MdlxData {
    version: u32,
    model: Model,
    sequences: Vec<Sequence>,
    #[dbg(fmt = "{:?}")] // compact
    globalseqs: Vec<u32>,
    textures: Vec<Texture>,
    materials: Vec<Material>,
    texanims: Vec<TextureAnim>,
    geosets: Vec<Geoset>,
    geoanims: Vec<GeosetAnim>,
}

pub struct MdlxChunk {
    pub id: u32,
    pub body: Vec<u8>,
}
impl MdlxChunk {
    pub fn parse_mdx(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        let id = cur.read_be()?;
        let sz = cur.readx()?;
        vvlog!("chunk = 0x{:08X} ({}) [{}]", id, u32_to_ascii(id), sz);
        let body = cur.read_bytes(sz)?;
        vvvlog!("{}", pretty_hex(&body).replace("\n", "\n\t"));
        return Ok(MdlxChunk { id, body });
    }
}

impl MdlxData {
    pub fn read(path: &Path) -> Result<Self, MyError> {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match ext {
            "mdl" => match std::fs::read_to_string(path) {
                Err(e) => ERR!("Failed to read file: {:?}, {}", path, e),
                Ok(s) => Self::parse_mdl(&s),
            },
            "mdx" => match std::fs::read(path) {
                Err(e) => ERR!("Failed to read file: {:?}, {}", path, e),
                Ok(s) => Self::parse_mdx(&s),
            },
            _ => ERR!("Invalid input path: {:?}, expected *.mdl or *.mdx", path),
        }
    }

    pub fn write(&self, _: &Path) -> Result<(), MyError> /* [todo] */ {
        return Ok(());
    }

    pub fn parse_mdl(_: &str) -> Result<Self, MyError> /* [todo] */ {
        let mut ret = MdlxData::default();
        return Ok(ret);
    }

    pub fn parse_mdx(input: &Vec<u8>) -> Result<Self, MyError> {
        let mut ret = MdlxData::default();
        let mut cur = Cursor::new(input);

        let magic = cur.read_be::<u32>()?;
        if magic != MdlxMagic::MDLX as u32 {
            return ERR!("Invalid magic: 0x{:08X}", magic);
        }

        while !cur.eol() {
            let chunk = MdlxChunk::parse_mdx(&mut cur)?;
            ret.parse_chunk(&chunk)?;
        }

        dbgx!(&ret); //[test]
        return Ok(ret);
    }

    fn parse_chunk(&mut self, chunk: &MdlxChunk) -> Result<(), MyError> /* [todo] */ {
        let mut cur = Cursor::new(&chunk.body);
        if chunk.id == MdlxMagic::VERS as u32 {
            self.version = cur.readx()?;
        } else if chunk.id == Model::ID {
            self.model = Model::parse_mdx(&mut cur)?;
        } else if chunk.id == Sequence::ID {
            while !cur.eol() {
                self.sequences.push(Sequence::parse_mdx(&mut cur)?);
            }
        } else if chunk.id == GlobalSequence::ID {
            while !cur.eol() {
                self.globalseqs.push(GlobalSequence::parse_mdx(&mut cur)?.duration);
            }
        } else if chunk.id == Texture::ID {
            while !cur.eol() {
                self.textures.push(Texture::parse_mdx(&mut cur)?);
            }
        } else if chunk.id == Material::ID {
            while !cur.eol() {
                let sz = cur.readx::<u32>()? - 4;
                let body = cur.read_bytes(sz)?;
                let mut cur2 = Cursor::new(&body);
                self.materials.push(Material::parse_mdx(&mut cur2)?);
            }
        } else if chunk.id == TextureAnim::ID {
            while !cur.eol() {
                let sz = cur.readx::<u32>()? - 4;
                let body = cur.read_bytes(sz)?;
                let mut cur2 = Cursor::new(&body);
                self.texanims.push(TextureAnim::parse_mdx(&mut cur2)?);
            }
        } else if chunk.id == Geoset::ID {
            while !cur.eol() {
                let sz = cur.readx::<u32>()? - 4;
                let body = cur.read_bytes(sz)?;
                vvvlog!("{}", pretty_hex(&body).replace("\n", "\n\t"));
                let mut cur2 = Cursor::new(&body);
                self.geosets.push(Geoset::parse_mdx(&mut cur2)?);
            }
            // } else if chunk.id == GeosetAnim::ID {
            //     while !cur.eol() {
            //         let sz = cur.readx::<u32>()? - 4;
            //         let body = cur.read_bytes(sz)?;
            //         let mut cur2 = Cursor::new(&body);
            //         self.geoanims.push(GeosetAnim::parse_mdx(&mut cur2)?);
            //     }
        }
        return Ok(());
    }
}
