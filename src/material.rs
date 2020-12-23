use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::random::Random;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

#[derive(Copy, Clone, Debug)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vec3
}

impl Material {
    fn reflect(self, v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn scatter(self, random: &mut Random, input: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let mut out: Option<ScatteredRay> = None;
        match self {
            Material::Lambertian(color) => {
                let mut direction = hit_record.normal + random.random_unit_vector();
                if direction.near_zero() {
                    direction = hit_record.normal;
                }
                out = Some(ScatteredRay {
                    ray: Ray { origin: hit_record.p, direction },
                    attenuation: color
                });
            },
            Material::Metal(color) => {
                let direction = self.reflect(input.direction.unit_vector(), hit_record.normal);
                let ray = ScatteredRay {
                    ray: Ray { origin: hit_record.p, direction },
                    attenuation: color
                };

                if ray.ray.direction.dot(hit_record.normal) > 0.0 {
                    out = Some(ray);
                }
            }
        }
        out
    }
}