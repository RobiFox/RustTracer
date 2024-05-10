mod vec3;
mod libs;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use crate::vec3::Vec3;

const IMAGE_WIDTH: u16 = 128;
const IMAGE_HEIGHT: u16 = 128;

fn main() {
    let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut stdout = stdout();
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let color = Vec3::new((j as f64) / ((IMAGE_WIDTH as f64)), (i as f64) / ((IMAGE_HEIGHT as f64)), 0.0);
            print!("\rRemaining: {} {}%    ", (IMAGE_HEIGHT - i - 1), ((j * 100) / IMAGE_WIDTH));
            stdout.flush().unwrap();
            contents.push_str(libs::write_color(&color).as_str());
        }
    }
    let mut file = File::create("image.ppm").expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write to file");

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}
