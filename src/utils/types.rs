use crate::*;

//#region trait: CheckDefault, CheckValue

pub trait _CheckDefault {
    fn isdef(&self) -> bool; //=::default()
}
impl<T> _CheckDefault for T
where
    T: Default + PartialEq,
{
    fn isdef(&self) -> bool {
        *self == T::default()
    }
}

#[allow(dead_code)]
pub trait CheckValue {
    fn is0(&self) -> bool; //=0
    fn is1(&self) -> bool; //=1
    fn isneg1(&self) -> bool; //=-1
}

macro_rules! impl_CheckValue {
    ($($t:ty),*) => {
        $(
            impl CheckValue for $t {
                fn is0(&self) -> bool {
                    *self == 0 as Self
                }
                fn is1(&self) -> bool {
                    *self == 1 as Self
                }
                fn isneg1(&self) -> bool {
                    *self == -1i8 as Self
                }
            }
            impl CheckValue for &[$t] {
                fn is0(&self) -> bool {
                    self.iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.iter().all(|x| x.isneg1())
                }
            }
            impl CheckValue for &Vec<$t> {
                fn is0(&self) -> bool {
                    self.iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.iter().all(|x| x.isneg1())
                }
            }
            impl CheckValue for Vec<$t> {
                fn is0(&self) -> bool {
                    self.iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.iter().all(|x| x.isneg1())
                }
            }
        )*
    };
}
macro_rules! impl_CheckValue_vecN {
    ($($t:ty),*) => {
        $(
            impl CheckValue for $t {
                fn is0(&self) -> bool {
                    self.to_array().iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.to_array().iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.to_array().iter().all(|x| x.isneg1())
                }
            }
            impl CheckValue for &[$t] {
                fn is0(&self) -> bool {
                    self.iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.iter().all(|x| x.isneg1())
                }
            }
            impl CheckValue for &Vec<$t> {
                fn is0(&self) -> bool {
                    self.iter().all(|x| x.is0())
                }
                fn is1(&self) -> bool {
                    self.iter().all(|x| x.is1())
                }
                fn isneg1(&self) -> bool {
                    self.iter().all(|x| x.isneg1())
                }
            }
        )*
    };
}

impl_CheckValue!(i8, u8, i16, u16, i32, u32, f32);
impl_CheckValue_vecN!(Vec2, Vec3, Vec4);

impl CheckValue for bool {
    fn is0(&self) -> bool {
        !*self
    }
    fn is1(&self) -> bool {
        *self
    }
    fn isneg1(&self) -> bool {
        false
    }
}
impl CheckValue for String {
    fn is0(&self) -> bool {
        self.is_empty()
    }
    fn is1(&self) -> bool {
        false
    }
    fn isneg1(&self) -> bool {
        false
    }
}
impl CheckValue for str {
    fn is0(&self) -> bool {
        self.is_empty()
    }
    fn is1(&self) -> bool {
        false
    }
    fn isneg1(&self) -> bool {
        false
    }
}

//#endregion
//#region convert

pub trait ConvertVec<A> {
    fn convert<B, F>(&self, f: F) -> Vec<B>
    where
        F: Fn(&A) -> B;
    fn try_convert<B, F>(&self, f: F) -> Result<Vec<B>, MyError>
    where
        F: Fn(&A) -> Result<B, MyError>;
}

impl<A> ConvertVec<A> for Vec<A> {
    fn convert<B, F>(&self, f: F) -> Vec<B>
    where
        F: Fn(&A) -> B,
    {
        self.into_iter().map(f).collect()
    }

    fn try_convert<B, F>(&self, f: F) -> Result<Vec<B>, MyError>
    where
        F: Fn(&A) -> Result<B, MyError>,
    {
        let mut ret = Vec::with_capacity(self.len());
        for a in self.iter() {
            ret.push(f(a)?);
        }
        return Ok(ret);
    }
}

//#endregion
