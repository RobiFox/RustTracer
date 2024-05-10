mod vec3;

use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;
use crate::vec3::Vec3;

const IMAGE_WIDTH: u16 = 128;
const IMAGE_HEIGHT: u16 = 128;

fn main() {
    let mut contents = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut stdout = stdout();
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r: f32 = (j as f32) / ((IMAGE_WIDTH) as f32);
            let g: f32 = (i as f32) / ((IMAGE_HEIGHT) as f32);
            let b: f32 = 0.0;

            let ir = (255.0 * r).floor() as u8;
            let ig = (255.0 * g).floor() as u8;
            let ib = (255.0 * b).floor() as u8;

            //println!("{ir} {ig} {ib}");
            print!("\rRemaining: {} {}%    ", (IMAGE_HEIGHT - i - 1), ((j * 100) / IMAGE_WIDTH));
            stdout.flush().unwrap();
            contents.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
    let mut file = File::create("image.ppm").expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write to file");

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}
