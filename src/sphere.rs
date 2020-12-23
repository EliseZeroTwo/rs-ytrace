use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_center = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = origin_center.dot(ray.direction);
        let c = origin_center.len_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt_discriminant = discriminant.sqrt();

        let root = (-half_b - sqrt_discriminant) / a;
        if t_min > root || root > t_max {
            let root = (-half_b + sqrt_discriminant) / a;
            if t_min > root || root > t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let mut record: HitRecord = HitRecord { t: root, p: p, normal: (p - self.center) / self.radius, front_facing: true };
        record.set_face_normal(ray);
        Some(record)
    }
}