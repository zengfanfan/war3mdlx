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

pub fn u32_to_ascii(n: u32) -> String {
    let bytes = n.to_be_bytes(); // 大端字节顺序，高位在前
    String::from_utf8_lossy(&bytes).into_owned()
}
