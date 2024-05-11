use std::f64::consts::PI;
use rand::Rng;
use crate::vec3::Vec3;

pub fn write_color(color: &Vec3) -> String {
    let ir = (255.999 * color.x).floor() as u8;
    let ig = (255.999 * color.y).floor() as u8;
    let ib = (255.999 * color.z).floor() as u8;

    format!("{} {} {}\n", ir, ig, ib)
}

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}

pub fn random_vector() -> Vec3 {
    random_vector_range(0.0,1.0)
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let unit_sphere = random_unit_vector();
    if unit_sphere.dot(normal) > 0.0 { unit_sphere } else { -unit_sphere }
}

pub fn random_vector_range(min: f64, max: f64) -> Vec3 {
    let mut rand = rand::thread_rng();
    Vec3::new(rand.gen_range(min..max), rand.gen_range(min..max), rand.gen_range(min..max))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}