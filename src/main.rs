mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod random;
mod ray;
mod raytracer;
mod sphere;
mod vec3;

extern crate image;
extern crate rand;

use crate::raytracer::Raytracer;
use image::EncodableLayout;

const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * (16.0 / 9.0)) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;

fn main() {
    let mut raytracer: Raytracer = Raytracer::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    image::save_buffer(
        "image.png",
        raytracer.generate_image().as_bytes(),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        image::ColorType::Rgb8,
    );
}
