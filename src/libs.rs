use std::f64::consts::PI;
use crate::vec3::Vec3;

pub fn write_color(color: &Vec3) -> String {
    let ir = (255.0 * color.x).floor() as u8;
    let ig = (255.0 * color.y).floor() as u8;
    let ib = (255.0 * color.z).floor() as u8;

    format!("{} {} {}\n", ir, ig, ib)
}

pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}