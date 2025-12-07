use crate::*;

pub trait _ExtendString {}

impl _ExtendString for String {}

#[allow(dead_code)]
pub trait _ExtendStringArray {
    fn push_if(&mut self, cond: bool, s: String);
}
impl _ExtendStringArray for Vec<String> {
    fn push_if(&mut self, cond: bool, s: String) {
        yes!(cond, self.push(s));
    }
}

#[allow(dead_code)]
pub trait _ExtendStringArrayDisplay<T> {
    fn push_if_n0(&mut self, name: &str, v: &T);
    fn push_if_n1(&mut self, name: &str, v: &T);
    fn push_if_nneg1(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Display> _ExtendStringArrayDisplay<T> for Vec<String> {
    fn push_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.push(F!("{} {},", name, v)));
    }
    fn push_if_n1(&mut self, name: &str, v: &T) {
        yes!(!v.is1(), self.push(F!("{} {},", name, v)));
    }
    fn push_if_nneg1(&mut self, name: &str, v: &T) {
        yes!(!v.isneg1(), self.push(F!("{} {},", name, v)));
    }
}

#[allow(dead_code)]
pub trait _ExtendStringArrayFMTX<T> {
    fn pushx(&mut self, name: &str, v: &T);
}
impl<T: Formatter> _ExtendStringArrayFMTX<T> for Vec<String> {
    fn pushx(&mut self, name: &str, v: &T) {
        self.push(F!("{} {},", name, v.fmt()));
    }
}

#[allow(dead_code)]
pub trait _ExtendStringArrayIfFMTX<T> {
    fn pushx_if_n0(&mut self, name: &str, v: &T);
    fn pushx_if_n1(&mut self, name: &str, v: &T);
    fn pushx_if_nneg1(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Formatter> _ExtendStringArrayIfFMTX<T> for Vec<String> {
    fn pushx_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.pushx(name, v));
    }
    fn pushx_if_n1(&mut self, name: &str, v: &T) {
        yes!(!v.is1(), self.pushx(name, v));
    }
    fn pushx_if_nneg1(&mut self, name: &str, v: &T) {
        yes!(!v.isneg1(), self.pushx(name, v));
    }
}
