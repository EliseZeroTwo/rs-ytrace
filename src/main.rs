mod vec3;
mod color;

extern crate image;

use image::EncodableLayout;
use crate::vec3::Vec3;
use crate::color::write_color;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;
const IMAGE_BYTES_PER_PIXEL: u32 = 3;

fn generate_test_image() -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for y in 0u32..IMAGE_HEIGHT {
        for x in 0u32..IMAGE_WIDTH {
            let mut color: Vec3 = Vec3((x as f64) / ((IMAGE_WIDTH - 1) as f64), (y as f64) / ((IMAGE_HEIGHT - 1) as f64), 0.25);
            write_color(&mut color, &mut out);
        }
    }
    out
}

fn main() {
    image::save_buffer("image.png", generate_test_image().as_bytes(), IMAGE_WIDTH, IMAGE_HEIGHT, image::ColorType::Rgb8);
}
