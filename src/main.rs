mod vec3;
mod color;
mod ray;

extern crate image;

use image::EncodableLayout;
use crate::vec3::Vec3;
use crate::color::write_color;
use crate::ray::Ray;

const APSECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 480;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / APSECT_RATIO) as u32;
const IMAGE_BYTES_PER_PIXEL: u32 = 3;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = APSECT_RATIO * VIEWPORT_HEIGHT; 
const FOCAL_LENGTH: f64 = 1.0;

pub fn hit_sphere(ray: Ray, sphere_center: Vec3, radius: f64) -> f64 {
    let origin_center = ray.origin - sphere_center;
    let a = ray.direction.len_squared();
    let half_b = origin_center.dot(ray.direction);
    let c = origin_center.len_squared() - (radius * radius);
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn ray_color(ray: Ray) -> Vec3 {
    const SPHERE_CENTER: Vec3 = Vec3(0.0, 0.0, -1.0);

    let t = hit_sphere(ray, Vec3(0.0, 0.0, -1.0), 0.5);
    if t > 0.0 {
        let normal = (ray.at(t) - SPHERE_CENTER).unit_vector();
        return 0.5 * (normal + 1.0);
    }
    
    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Vec3(1.0, 1.0, 1.0)) +  (t * Vec3(1.0, 0.6, 1.7))
}

fn generate_image() -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let bottom_left_point: Vec3 = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3(0.0, 0.0, FOCAL_LENGTH);

    for y in 0u32..IMAGE_HEIGHT {
        println!("At (0, {}), {}  line(s) remain!", y, IMAGE_HEIGHT - y);
        for x in 0u32..IMAGE_WIDTH {
            let horizonal_offset = (x as f64) / ((IMAGE_WIDTH - 1) as f64);
            let vertical_offset = (y as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let ray = Ray { origin: origin, direction: bottom_left_point + (horizontal * horizonal_offset) + (vertical * vertical_offset) - origin };
            let color = ray_color(ray);
            write_color(&color, &mut out);
        }
    }
    out
}

fn main() {
    image::save_buffer("image.png", generate_image().as_bytes(), IMAGE_WIDTH, IMAGE_HEIGHT, image::ColorType::Rgb8);
}
