mod vec3;
mod libs;
mod ray;
mod hittable;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

const IMAGE_WIDTH: u32 = 350;
const IMAGE_HEIGHT: u32 = 250;

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut hit_record = HitRecord::empty();
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0))
    }

    let unit = ray.direction.normalize();
    let a = 0.5 * (unit.y + 1.0);
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
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

    let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world: HittableList = HittableList { vec: vec![] };
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.5),
            0.5,
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.5, -1.0),
            0.5,
        ))
    );

    use std::time::Instant;
    let now = Instant::now();

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            //print!("\rRemaining: {} {}%    ", (IMAGE_HEIGHT - j - 1), ((i * 100) / IMAGE_WIDTH));
            //stdout.flush().unwrap();

            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            //let color = Vec3::new((j as f64) / ((IMAGE_WIDTH as f64)), (i as f64) / ((IMAGE_HEIGHT as f64)), 0.0);
            let color = ray_color(&r, &world);
            contents.push_str(libs::write_color(&color).as_str());
        }
    }
    print!("\n");
    let mut file = File::create("image.ppm").expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write to file");
    let elapsed = now.elapsed();
    println!("Took {:?} seconds to generate frame.", elapsed);

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}