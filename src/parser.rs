use crate::*;

#[derive(Dbg, Default)]
pub struct MdlxData {
    version: Version,
    model: Model,
    sequences: Vec<Sequence>,
    #[dbg(formatter = "fmtx")]
    globalseqs: Vec<GlobalSequence>,
    textures: Vec<Texture>,
    materials: Vec<Material>,
    texanims: Vec<TextureAnim>,
    geosets: Vec<Geoset>,
    geoanims: Vec<GeosetAnim>,
    #[dbg(formatter = "fmtx")]
    pivot_points: Vec<PivotPoint>,
    cameras: Vec<Camera>,
    bones: Vec<Bone>,
    helpers: Vec<Helper>,
    attachments: Vec<Attachment>,
    collisions: Vec<CollisionShape>,
    lights: Vec<Light>,
    eventobjs: Vec<EventObject>,
    particle_emitters: Vec<ParticleEmitter>,
    particle_emitters2: Vec<ParticleEmitter2>,
    ribbon_emitters: Vec<RibbonEmitter>,
}

macro_rules! MdxReadChunkType1 {
    ($chunk:expr, $cur:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $chunk.id == <$ty>::ID {
            $var = <$ty>::read_mdx(&mut $cur)
            .or_else(|e| ERR!("{}({}): {}", TNAME!($ty), u32_to_ascii(<$ty>::ID), e))?;
            return Ok(());
        })+
    };
}
macro_rules! MdxReadChunkType2 {
    ($chunk:expr, $cur:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $chunk.id == <$ty>::ID {
            while !$cur.eol() {
                $var.push(
                    <$ty>::read_mdx(&mut $cur).or_else(|e| {
                        ERR!("{}({})[{}th]: {}", TNAME!($ty), u32_to_ascii(<$ty>::ID), $var.len(), e)
                    })?,
                );
            }
            return Ok(());
        })+
    };
}
macro_rules! MdxReadChunkType3 {
    ($chunk:expr, $cur:expr, $( $ty:ty => $var:expr ),+ $(,)?) => {
        $(if $chunk.id == <$ty>::ID {
            while !$cur.eol() {
                let left = $cur.left();
                yes!(left < 4, EXIT!("{} size: {}B left (need 4)", TNAME!($ty), left));
                let sz = $cur.readx::<u32>()?;
                yes!(sz < 4, EXIT!("{} size: {} (need >= 4)", TNAME!($ty), sz));
                let sz = sz - 4;

                let left = $cur.left();
                yes!(left < sz, EXIT!("{} body: {}B left (need {})", TNAME!($ty), left, sz));
                let body = $cur.read_bytes(sz).or_else(|e| ERR!("{} body({}B): {}", TNAME!($ty), sz, e))?;

                let mut cur2 = Cursor::new(&body);
                $var.push(
                    <$ty>::read_mdx(&mut cur2).or_else(|e| {
                        ERR!("{}({})[{}th]: {}", TNAME!($ty), u32_to_ascii(<$ty>::ID), $var.len(), e)
                    })?,
                );
            }
            EXIT!();
        })+
    };
}

#[macro_export]
macro_rules! MdlWriteType1 {
    ($lines:ident, $depth:expr, $( $var:expr ),+ $(,)?) => {
        $( $lines.append(&mut $var.write_mdl($depth)?); )+
    };
}
#[macro_export]
macro_rules! MdlWriteType2 {
    ($lines:ident, $depth:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            let indent = indent!($depth);
            $lines.push(F!("{indent}{} {} {{", $name, $var.len()));
            for a in &$var {
                MdlWriteType1!($lines, $depth+1, a);
            }
            $lines.push(F!("{indent}}}"));
        })+
    };
}
#[macro_export]
macro_rules! MdlWriteType3 {
    ($lines:ident, $depth:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            for a in &$var {
                $lines.push(F!("{} {{", $name));
                MdlWriteType1!($lines, $depth+1, a);
                $lines.push(F!("}}"));
            }
        })+
    };
}
#[macro_export] // Nodes
macro_rules! MdlWriteType4 {
    ($lines:ident, $depth:expr, $member:expr, $( $name:expr => $var:expr ),+ $(,)?) => {
        $(if !$var.is_empty() {
            for a in &$var {
                paste!{ $lines.push(F!("{} \"{}\" {{", $name, a.$member)); }
                MdlWriteType1!($lines, $depth+1, a);
                $lines.push(F!("}}"));
            }
        })+
    };
}

impl MdlxData {
    pub fn read(path: &Path) -> Result<Self, MyError> {
        match path.ext_lower().as_str() {
            "mdl" => match std::fs::read_to_string(path) {
                Err(e) => ERR!("Failed to read file {:?}: {}", path, e),
                Ok(s) => Self::read_mdl(&s).or_else(|e| ERR!("Failed to read file {:?}: {}", path, e)),
            },
            "mdx" => match std::fs::read(path) {
                Err(e) => ERR!("Failed to read file {:?}: {}", path, e),
                Ok(s) => Self::read_mdx(&s).or_else(|e| ERR!("Failed to read file {:?}: {}", path, e)),
            },
            _ => ERR!("Invalid input path: {:?}, expected *.mdl or *.mdx", path),
        }
    }

    pub fn write(&mut self, path: &Path) -> Result<(), MyError> {
        match path.ext_lower().as_ref() {
            "mdl" => self.write_mdl(path).or_else(|e| ERR!("Failed to write file {:?}: {}", path, e)),
            "mdx" => self.write_mdx(path).or_else(|e| ERR!("Failed to write file {:?}: {}", path, e)),
            _ => ERR!("Invalid output path: {:?}, expected *.mdl or *.mdx", path),
        }
    }

    pub fn write_mdl(&mut self, path: &Path) -> Result<(), MyError> {
        let mut lines: Vec<String> = vec![];

        MdlWriteType1!(lines, 0, self.version, self.model);
        MdlWriteType2!(lines, 0,
            "Sequences" => self.sequences,
            "GlobalSequences" => self.globalseqs,
            "Textures" => self.textures,
            "Materials" => self.materials,
            "TextureAnims" => self.texanims,
        );
        MdlWriteType3!(lines, 0,
            "Geoset" => self.geosets,
            "GeosetAnim" => self.geoanims,
        );
        MdlWriteType4!(lines, 0, base.name,
            "Bone" => self.bones,
            "Light" => self.lights,
            "Helper" => self.helpers,
            "Attachment" => self.attachments,
            "ParticleEmitter" => self.particle_emitters,
            "ParticleEmitter2" => self.particle_emitters2,
            "RibbonEmitter" => self.ribbon_emitters,
            "EventObject" => self.eventobjs,
            "CollisionShape" => self.collisions,
        );
        MdlWriteType2!(lines, 0, "PivotPoints" => self.pivot_points );
        MdlWriteType4!(lines, 0, name, "Camera" => self.cameras );

        let text = lines.join("\n") + "\n";
        return Ok(std::fs::write(path, text)?);
    }

    pub fn write_mdx(&self, _: &Path) -> Result<(), MyError> /* [todo] */ {
        todo!();
    }

    pub fn read_mdl(_: &str) -> Result<Self, MyError> /* [todo] */ {
        todo!();
    }

    pub fn read_mdx(input: &Vec<u8>) -> Result<Self, MyError> {
        let mut this = MdlxData::default();
        let mut cur = Cursor::new(input);

        let magic = cur.read_be::<u32>().unwrap_or(0);
        if magic != MdlxMagic::MDLX as u32 {
            return ERR!("Invalid magic: 0x{:08X} ({})", magic, u32_to_ascii(magic));
        }

        while !cur.eol() {
            let chunk = MdxChunk::read_mdx(&mut cur)?;
            this.parse_chunk(&chunk)?;
        }

        let format_version = this.version.format_version;
        if !Version::SUPPORTED_VERSION.contains(&format_version) {
            EXIT!("Unsupported version: {} (should be one of {:?})", format_version, Version::SUPPORTED_VERSION);
        }

        for (i, a) in this.attachments.iter_mut().enumerate() {
            a.aindex = i as i32;
        }

        dbgx!(&this);
        return Ok(this);
    }

    fn parse_chunk(&mut self, chunk: &MdxChunk) -> Result<(), MyError> {
        let mut cur = Cursor::new(&chunk.body);
        MdxReadChunkType1!(chunk, cur,
            Version => self.version,
            Model   => self.model,
        );
        MdxReadChunkType2!(chunk, cur,
            Sequence        => self.sequences,
            GlobalSequence  => self.globalseqs,
            Texture         => self.textures,
            Bone            => self.bones,
            Helper          => self.helpers,
            EventObject     => self.eventobjs,
            CollisionShape  => self.collisions,
            PivotPoint      => self.pivot_points,
        );
        MdxReadChunkType3!(chunk, cur,
            TextureAnim     => self.texanims,
            Material        => self.materials,
            Geoset          => self.geosets,
            GeosetAnim      => self.geoanims,
            Attachment      => self.attachments,
            Light           => self.lights,
            ParticleEmitter => self.particle_emitters,
            ParticleEmitter2=> self.particle_emitters2,
            RibbonEmitter   => self.ribbon_emitters,
            Camera          => self.cameras,
        );
        return Ok(());
    }
}
