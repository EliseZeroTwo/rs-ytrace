use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    bottom_left: Vec3
}

impl Camera {
    pub fn new(apsect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(apsect_ratio * viewport_height, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        Camera {
            origin,
            horizontal,
            vertical,
            bottom_left: origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3(0.0, 0.0, focal_length)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.bottom_left + (u * self.horizontal) + (v * self.vertical) - self.origin
        }
    }
}