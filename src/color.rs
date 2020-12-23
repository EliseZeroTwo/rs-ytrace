use crate::vec3::Vec3;

pub fn write_color(color: &Vec3, out: &mut Vec<u8>, samples_per_pixel: u32) {
    let mut color = *color;
    let scale = 1.0 / samples_per_pixel as f64;
    color *= scale;

    out.push((color.x().clamp(0.0, 0.999) * 256.0).floor() as u8);
    out.push((color.y().clamp(0.0, 0.999) * 256.0).floor() as u8);
    out.push((color.z().clamp(0.0, 0.999) * 256.0).floor() as u8);
}