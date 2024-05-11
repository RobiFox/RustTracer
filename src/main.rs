mod vec3;
mod libs;
mod ray;
mod hittable;
mod camera;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::vec3::{Vec3};

const IMAGE_WIDTH: u32 = 350;
const IMAGE_HEIGHT: u32 = 250;

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
            Vec3::new(0.0, 0.5, -1.5),
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