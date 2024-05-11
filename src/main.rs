mod vec3;
mod libs;
mod ray;
mod hittable;
mod camera;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::ray::Ray;
use crate::vec3::{Vec3};

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

    let camera = Camera::new(1.0);
    camera.render(&world);

    let elapsed = now.elapsed();
    println!("Took {:?} seconds to generate frame.", elapsed);

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}