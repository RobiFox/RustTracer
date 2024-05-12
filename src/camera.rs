use std::fs::File;
use std::io::Write;
use rand::Rng;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, libs};
use crate::libs::{degrees_to_radians};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    position: Vec3,
    forward: Vec3,
    world_up: Vec3,
    u: Vec3,
    v: Vec3,

    focal_length: f64,
    camera_center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    //viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
    max_bounces: u8,
    samples_per_pixel: u8,
    pixel_sample_scale: f64,
    vfov: f64,
}

impl Camera {
    pub fn render(&self, world: &HittableList) -> &Self {
        //self.initialize();
        let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.construct_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, &world, self.max_bounces);
                }

                let color = self.pixel_sample_scale * pixel_color;
                contents.push_str(libs::write_color(&color).as_str());
            }
        }

        let mut file = File::create("image.ppm").expect("Unable to create file");
        file.write_all(contents.as_bytes())
            .expect("Unable to write to file");
        self
    }

    fn construct_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_center = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_center - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn recalculate_camera_vectors(&mut self) {

    }

    fn initialize(mut self) -> Self {
        self.recalculate_camera_vectors();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focal_length;
        let viewport_width = viewport_height * ((IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64));
        self.camera_center = self.position;
        self.v = self.world_up.normalize();
        self.u = self.forward.cross(&self.v).normalize();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0) * self.u;
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0) * -self.v;

        self.pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
        self.pixel_delta_v = viewport_v / (IMAGE_HEIGHT as f64);

        let viewport_upper_left = self.camera_center
            - (self.forward * self.focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        self
    }

    fn ray_color(ray: &Ray, world: &HittableList, bounces_left: u8) -> Vec3 {
        let mut hit_record = HitRecord::empty();
        if bounces_left > 0 && world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
            if let Some(material) = hit_record.material.as_ref() {
                if material.scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
                    return Self::ray_color(&scattered, world, bounces_left - 1) * attenuation;
                }
            }
            //return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit = ray.direction.normalize();
        let a = 0.5 * (unit.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }

    fn sample_square() -> Vec3 {
        let mut rand = rand::thread_rng();
        Vec3::new(rand.gen_range(-0.5..0.5), rand.gen_range(-0.5..0.5), 0.0)
    }

    pub fn new(position: Vec3, focal_length: f64, max_bounces: u8, samples_per_pixel: u8, vfov: f64) -> Camera {
        let camera = Camera {
            position,
            forward: Vec3::new(0.0, 0.0, 1.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),

            focal_length,
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            camera_center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_sample_scale: 1.0 / (samples_per_pixel as f64),
            max_bounces,
            samples_per_pixel,
            vfov,
        };
        camera.initialize()
    }
}