use crate::vec3::Vec3;

pub fn write_color(color: &mut Vec3, out: &mut Vec<u8>) {
    out.push((color.x() * 255.999).floor() as u8);
    out.push((color.y() * 255.999).floor() as u8);
    out.push((color.z() * 255.999).floor() as u8);
}