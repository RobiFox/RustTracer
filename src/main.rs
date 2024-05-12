mod vec3;
mod libs;
mod ray;
mod hittable;
mod camera;
mod material;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::Material;
use crate::vec3::{Vec3};

const IMAGE_WIDTH: u32 = 350 * 2;
const IMAGE_HEIGHT: u32 = 250 * 2;

fn main() {
    let mut world: HittableList = HittableList { vec: vec![] };
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Some(Material::Lambertian { albedo: Vec3::new(1.0, 0.1, 0.1) })
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.5),
            0.5,
            Some(Material::Lambertian { albedo: Vec3::new(0.5, 1.0, 0.0) })
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(-2.0, 0.0, -1.5),
            1.5,
            Some(Material::Metal { albedo: Vec3::new(1.0, 1.0, 1.0), fuzziness: 1.0})
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(2.0, 0.0, -1.5),
            0.5,
            Some(Material::Dialectric { albedo: Vec3::new(1.0, 1.0, 1.0), refraction_index: 1.0 / 1.33})
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(2.0, 0.0, -1.5),
            0.75,
            Some(Material::Dialectric { albedo: Vec3::new(1.0, 1.0, 1.0), refraction_index: 1.33})
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(2.0, 1.5, -5.0),
            1.5,
            Some(Material::Lambertian { albedo: Vec3::new(0.5, 1.0, 0.0) })
        ))
    );

    use std::time::Instant;
    let now = Instant::now();

    let camera = Camera::new(1.0, 4, 8);
    camera.render(&world);

    let elapsed = now.elapsed();
    println!("Took {:?} seconds to generate frame.", elapsed);

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}