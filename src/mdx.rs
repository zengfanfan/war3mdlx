use crate::*;

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

impl MdlxData {
    pub fn write_mdx(&self, _: &Path) -> Result<(), MyError> /* [todo] */ {
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
            this.parse_mdx_chunk(&chunk)?;
        }

        dbgx!(&this);
        return Ok(this);
    }

    fn parse_mdx_chunk(&mut self, chunk: &MdxChunk) -> Result<(), MyError> {
        let mut cur = Cursor::new(&chunk.body);
        MdxReadChunkType1!(chunk, cur,
            Version         => self.version,
            Model           => self.model,
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
