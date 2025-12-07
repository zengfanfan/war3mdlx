use crate::*;

pub trait _ExtendString {}

impl _ExtendString for String {}

pub trait _ExtendStringArray<T> {
    fn push_if_n0(&mut self, name: &str, v: &T);
    fn push_if_nneg1(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Formatter> _ExtendStringArray<T> for Vec<String> {
    fn push_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.push(F!("{} {},", name, v.fmt())));
    }
    fn push_if_nneg1(&mut self, name: &str, v: &T) {
        yes!(!v.isneg1(), self.push(F!("{} {},", name, v.fmt())));
    }
}

pub trait _ExtendStringArrayX<T> {
    fn pushx(&mut self, name: &str, v: &T);
    fn pushx_if_n0(&mut self, name: &str, v: &T);
    fn pushx_if_n1(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Formatter> _ExtendStringArrayX<T> for Vec<String> {
    fn pushx(&mut self, name: &str, v: &T) {
        self.push(F!("{} {},", name, v.fmt()))
    }
    fn pushx_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.push(F!("{} {},", name, v.fmt())));
    }
    fn pushx_if_n1(&mut self, name: &str, v: &T) {
        yes!(!v.is1(), self.push(F!("{} {},", name, v.fmt())));
    }
}
