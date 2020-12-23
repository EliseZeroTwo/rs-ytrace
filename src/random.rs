use rand::rngs::SmallRng;
use rand::RngCore;
use crate::vec3::Vec3;


pub struct Random(pub SmallRng);

impl Random {
    pub fn f64_random(&mut self) -> f64 {
        (self.0.next_u64() as f64 / 100000.0) % 1.0
    }

    pub fn f64_random_bounded(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.f64_random()
    }

    pub fn random(&mut self) -> Vec3 {
        Vec3(self.f64_random(), self.f64_random(), self.f64_random())
    }

    pub fn random_bounded(&mut self, min: f64, max: f64) -> Vec3 {
        Vec3(
            self.f64_random_bounded(min, max),
            self.f64_random_bounded(min, max),
            self.f64_random_bounded(min, max),
        )
    }

    pub fn random_in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let p = self.random_bounded(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(&mut self) -> Vec3 {
        self.random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(&mut self, normal: &Vec3) -> Vec3 {
        let mut unit_sphere = self.random_in_unit_sphere();
        if unit_sphere.dot(*normal) <= 0.0 {
            unit_sphere = -unit_sphere;
        }
        unit_sphere
    }
}