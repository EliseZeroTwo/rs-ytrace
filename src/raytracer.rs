use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct Raytracer {
    aspect_ratio: f64,
    image_height: u32,
    image_width: u32,
    bytes_per_pixel: u32,
    samples_per_pixel: u32,
    max_reflection_recursion: u32,

    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,

    rng_source: SmallRng,
    camera: Camera,
    pub world: HittableList,
}

impl Raytracer {
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32) -> Raytracer {
        let aspect_ratio = image_width as f64 / image_height as f64;
        let bytes_per_pixel = 3;
        let max_reflection_recursion = 50;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Raytracer {
            aspect_ratio,
            image_height,
            image_width,
            bytes_per_pixel,
            samples_per_pixel,
            max_reflection_recursion,

            viewport_height,
            viewport_width,
            focal_length,

            rng_source: SmallRng::from_entropy(),
            camera: Camera::new(aspect_ratio, viewport_height, focal_length),
            world: HittableList(Vec::new()),
        }
    }

    pub fn f64_random(&mut self) -> f64 {
        (self.rng_source.next_u64() as f64 / 100000.0) % 1.0
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

    fn ray_color(&mut self, ray: Ray, hittable: &dyn Hittable, depth: &mut u32) -> Vec3 {
        if *depth >= self.max_reflection_recursion {
            return Vec3(0.0, 0.0, 0.0);
        }
        *depth += 1;

        let opt_hit_record = hittable.hit(ray, 0.001, f64::MAX);
        match opt_hit_record {
            Some(hit_record) => {
                let target = hit_record.p + hit_record.normal + self.random_in_unit_sphere();
                0.5 * self.ray_color(
                    Ray {
                        origin: hit_record.p,
                        direction: target - hit_record.p,
                    },
                    hittable,
                    depth,
                )
            }
            None => {
                let unit_direction: Vec3 = ray.direction.unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                (t * Vec3(1.0, 0.6, 0.7)) + ((1.0 - t) * 1.0)
            }
        }
    }

    pub fn generate_image(&mut self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        let mut world: HittableList = HittableList(Vec::new());
        world.add(Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }));
        world.add(Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }));

        let camera = Camera::new(self.aspect_ratio, self.viewport_height, self.focal_length);

        for y in 0u32..self.image_height {
            println!("At (0, {}), {}  line(s) remain!", y, self.image_height - y);
            for x in 0u32..self.image_width {
                let mut color = Vec3(0.0, 0.0, 0.0);
                for s in 0u32..self.samples_per_pixel {
                    let horizonal_offset =
                        (x as f64 + self.f64_random()) / (self.image_width - 1) as f64;
                    let vertical_offset = ((self.image_height - 1 - y) as f64 + self.f64_random())
                        / (self.image_height - 1) as f64;
                    let ray = camera.get_ray(horizonal_offset, vertical_offset);
                    color += self.ray_color(ray, &mut world, &mut 0);
                }
                write_color(&color, &mut out, self.samples_per_pixel);
            }
        }
        out
    }
}
