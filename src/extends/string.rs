use crate::*;

pub trait _ExtendString {}

impl _ExtendString for String {}

pub trait _ExtendStringArray<T> {
    fn push_if(&mut self, cond: bool, v: &T);
    fn push_if_n0(&mut self, name: &str, v: &T);
    fn push_if_nneg1(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Display> _ExtendStringArray<T> for Vec<String> {
    fn push_if(&mut self, cond: bool, v: &T) {
        yes!(cond, self.push(v.to_string()));
    }
    fn push_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.push(F!("{} {},", name, v)));
    }
    fn push_if_nneg1(&mut self, name: &str, v: &T) {
        yes!(!v.isneg1(), self.push(F!("{} {},", name, v)));
    }
}

pub trait _ExtendStringArrayX<T> {
    fn pushx_if_n0(&mut self, name: &str, v: &T);
}
impl<T: CheckValue + Formatter> _ExtendStringArrayX<T> for Vec<String> {
    fn pushx_if_n0(&mut self, name: &str, v: &T) {
        yes!(!v.is0(), self.push(F!("{} {},", name, v.fmt())));
    }
}
