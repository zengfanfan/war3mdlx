use crate::*;

pub trait _ExtendVector {
    fn reverse(&self) -> Self;
}

impl<T: Clone + Copy> _ExtendVector for Vec<T> {
    fn reverse(&self) -> Self {
        let mut v: Self = Self::with_capacity(self.len());
        for i in 0..self.len() {
            v.push(self[self.len() - i - 1]);
        }
        return v;
    }
}
impl _ExtendVector for Vec2 {
    fn reverse(&self) -> Self {
        Self::new(self.y, self.x)
    }
}
impl _ExtendVector for Vec3 {
    fn reverse(&self) -> Self {
        Self::new(self.z, self.y, self.x)
    }
}
impl _ExtendVector for Vec4 {
    fn reverse(&self) -> Self {
        Self::new(self.w, self.z, self.y, self.x)
    }
}
