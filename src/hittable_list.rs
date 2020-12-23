use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct HittableList(pub Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.0.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut out: Option<HitRecord> = None;
        let mut closest: f64 = t_max;

        for x in &self.0 {
            let rec = x.hit(ray, t_min, closest);
            if let Some(hit_record) = rec {
                closest = hit_record.t;
                out = rec;
            }
        }
        out
    }
}
