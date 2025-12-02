use crate::*;

pub trait Extend_Cursor {
    fn readx<T: ReadFromCursor>(&mut self) -> Result<T, MyError>;
    fn read_le<T: ReadFromCursor>(&mut self) -> Result<T, MyError>;
    fn read_be<T: ReadFromCursor>(&mut self) -> Result<T, MyError>;
    fn read_if<T: ReadFromCursor>(&mut self, cond: bool, def: T) -> Result<T, MyError>;
    fn read_bytes(&mut self, n: u32) -> Result<Vec<u8>, MyError>;
    fn read_array<T: ReadFromCursor>(&mut self, n: u32) -> Result<Vec<T>, MyError>;
    fn read_array_be<T: ReadFromCursor>(&mut self, n: u32) -> Result<Vec<T>, MyError>;
    fn read_array_to<T: ReadFromCursor>(&mut self, a: &mut Vec<T>, n: u32) -> Result<(), MyError>;
    fn read_string(&mut self, n: u32) -> Result<String, MyError>;

    fn pos(&mut self) -> uint;
    fn len(&mut self) -> uint;
    fn left(&mut self) -> uint;
    fn eol(&mut self) -> bool;
}

// 2. 为 Cursor<Vec<u8>> 实现这个 trait
impl Extend_Cursor for Cursor<&Vec<u8>> {
    fn readx<T: ReadFromCursor>(&mut self) -> Result<T, MyError> {
        Ok(T::read_from(self)?)
    }
    fn read_le<T: ReadFromCursor>(&mut self) -> Result<T, MyError> {
        Ok(T::read_from(self)?)
    }
    fn read_be<T: ReadFromCursor>(&mut self) -> Result<T, MyError> {
        Ok(T::read_from_be(self)?)
    }

    fn read_bytes(&mut self, n: u32) -> Result<Vec<u8>, MyError> {
        let mut body = vec![0u8; n as usize];
        self.read_exact(&mut body)?;
        return Ok(body);
    }

    fn read_array<T: ReadFromCursor>(&mut self, n: u32) -> Result<Vec<T>, MyError> {
        let mut v = Vec::with_capacity(n as usize);
        for _ in 0..n {
            v.push(T::read_from(self)?);
        }
        return Ok(v);
    }
    fn read_array_be<T: ReadFromCursor>(&mut self, n: u32) -> Result<Vec<T>, MyError> {
        let mut v = Vec::with_capacity(n as usize);
        for _ in 0..n {
            v.push(T::read_from_be(self)?);
        }
        return Ok(v);
    }

    fn read_array_to<T: ReadFromCursor>(&mut self, v: &mut Vec<T>, n: u32) -> Result<(), MyError> {
        for _ in 0..n {
            v.push(T::read_from(self)?);
        }
        return Ok(());
    }

    fn read_string(&mut self, n: u32) -> Result<String, MyError> {
        let buf = self.read_bytes(n)?;
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let s = str::from_utf8(&buf[..end]).expect("Invalid UTF-8");
        return Ok(s.to_string());
    }

    fn read_if<T: ReadFromCursor>(&mut self, cond: bool, def: T) -> Result<T, MyError> {
        if cond { Ok(T::read_from(self)?) } else { Ok(def) }
    }

    fn pos(&mut self) -> uint {
        self.position() as uint
    }
    fn len(&mut self) -> uint {
        self.get_ref().len() as uint
    }
    fn left(&mut self) -> uint {
        self.len() - self.pos()
    }
    fn eol(&mut self) -> bool {
        self.position() >= self.get_ref().len() as u64
    }
}

//#region trait: ReadFromCursor

pub trait ReadFromCursor: Sized {
    fn read_from(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError>;
    fn read_from_be(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError>;
}

macro_rules! impl_ReadFromCursor {
    ($($a:ty),*) => {
        $(
            paste! {
                impl ReadFromCursor for $a {
                    fn read_from(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
                        Ok(cur.[<read_ $a>]::<LittleEndian>()?)
                    }
                    fn read_from_be(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
                        Ok(cur.[<read_ $a>]::<BigEndian>()?)
                    }
                }
            }
        )*
    };
}
macro_rules! impl_ReadFromCursor_VecN {
    ($($a:tt),*) => {
        $(
            paste! {
                impl ReadFromCursor for [<Vec $a>] {
                    fn read_from(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
                        let vs = cur.read_array::<f32>($a)?;
                        Ok(Self::from_slice(vs.as_slice()))
                    }
                    fn read_from_be(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
                        let vs = cur.read_array_be::<f32>($a)?;
                        Ok(Self::from_slice(vs.as_slice()))
                    }
                }
            }
        )*
    };
}

impl_ReadFromCursor!(i16, u16, i32, u32, f32);
impl_ReadFromCursor_VecN!(2, 3, 4);

impl ReadFromCursor for u8 {
    fn read_from(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(cur.read_u8()?)
    }
    fn read_from_be(cur: &mut Cursor<&Vec<u8>>) -> Result<Self, MyError> {
        Ok(cur.read_u8()?)
    }
}

//#endregion
