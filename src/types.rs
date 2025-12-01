use crate::*;

pub fn fmt_vec2(v: &Vec2) -> String {
    format!("({}, {})", v.x, v.y)
}
pub fn fmt_vec3(v: &Vec3) -> String {
    format!("({}, {}, {})", v.x, v.y, v.z)
}
pub fn fmt_vec4(v: &Vec4) -> String {
    format!("({}, {}, {}, {})", v.x, v.y, v.z, v.w)
}
