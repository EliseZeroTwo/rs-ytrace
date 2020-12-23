use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_facing: bool,
    pub material: Material
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray) {
        self.front_facing = ray.direction.dot(self.normal) < 0.0;
        if !self.front_facing {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
