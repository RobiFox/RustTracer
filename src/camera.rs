use std::fs::File;
use std::io::Write;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, libs, ray_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/*
let aspect_ratio = IMAGE_WIDTH / IMAGE_HEIGHT;

    let focal_length = 1.0;
    let viewport_height = -2.0;
    let viewport_width = viewport_height * ((IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
    let pixel_delta_v = viewport_v / (IMAGE_HEIGHT as f64);

    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
 */

pub struct Camera {
    focal_length: f64,
    camera_center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    //viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn render(&self, world: &HittableList) -> &Self {
        //self.initialize();
        let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {
                let r = self.construct_ray(i, j);

                let color = ray_color(&r, &world);
                contents.push_str(libs::write_color(&color).as_str());
            }
        }

        let mut file = File::create("image.ppm").expect("Unable to create file");
        file.write_all(contents.as_bytes())
            .expect("Unable to write to file");
        self
    }

    fn construct_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let ray_direction = pixel_center - self.camera_center;
        Ray::new(self.camera_center, ray_direction)
    }

    fn initialize(mut self) -> Self {
        self.focal_length = 1.0;
        let viewport_height = -2.0;
        let viewport_width = viewport_height * ((IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64));
        self.camera_center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
        self.pixel_delta_v = viewport_v / (IMAGE_HEIGHT as f64);

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        self
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
        let mut hit_record = HitRecord::empty();
        if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
            return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0))
        }

        let unit = ray.direction.normalize();
        let a = 0.5 * (unit.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }

    pub fn new(focal_length: f64) -> Camera {
        let mut camera = Camera {
            focal_length,
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            camera_center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.initialize()
    }
}