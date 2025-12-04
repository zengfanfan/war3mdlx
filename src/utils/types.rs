use crate::*;

//#region conditional operator

#[macro_export]
macro_rules! yesno {
    ($cond:expr, $y:expr, $n:expr) => {{ if $cond { $y } else { $n } }};
}

#[macro_export]
macro_rules! yes {
    ($cond:expr, $y:stmt) => {{
        if $cond {
            $y
        }
    }};
}

#[macro_export]
macro_rules! no {
    ($cond:expr, $n:stmt) => {{
        if !($cond) {
            $n
        }
    }};
}

//#endregion
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

//#endregion
