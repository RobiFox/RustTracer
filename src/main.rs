mod vec3;
mod libs;
mod ray;
mod hittable;
mod camera;
mod material;

use pixel_canvas::{Canvas, Color};
use pixel_canvas::canvas::CanvasInfo;
use pixel_canvas::input::{Event, MouseState, WindowEvent};
use pixel_canvas::input::glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere};
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
            Vec3::new(1.0, 0.5, -1.5),
            0.5,
            Some(Material::Dialectric { albedo: Vec3::new(1.0, 1.0, 1.0), refraction_index: 1.0 / 1.33})
        ))
    );
    world.vec.push(
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.5, -1.5),
            0.75,
            Some(Material::Dialectric { albedo: Vec3::new(1.0, 1.0, 1.0), refraction_index: 1.33})
        ))
    );

    use std::time::Instant;
    let now = Instant::now();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 1.0, 4, 255, 90.0);
    let mut pixels = vec![];
    let mut need_update = true;

    let canvas = Canvas::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
        .title("Ray Trace")
        .state(MouseState::new())
        .input(handle_input);
    canvas.render(move |mouse, image| {
        if !need_update {
            return;
        }
        pixels = camera.render(&world);
        need_update = false;
        for (x, row) in image.chunks_mut(IMAGE_WIDTH as usize).enumerate() {
            for (y, pixel) in row.iter_mut().enumerate() {
                let r = (&pixels[x][y].x * 255.999) as u8;
                let g = (&pixels[x][y].y * 255.999) as u8;
                let b = (&pixels[x][y].z * 255.999) as u8;
                *pixel = Color {
                    r, g, b
                }
            }
        }
    });

    let elapsed = now.elapsed();
    println!("Took {:?} seconds to generate frame.", elapsed);

    let _vector = Vec3::new(1.0, 2.0, 3.0);
}

pub fn handle_input(info: &CanvasInfo, mouse: &mut MouseState, event: &Event<()>) -> bool {
    return match event {
        Event::WindowEvent {
            event,
            ..
        } => {
            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    println!("input: {:?}", input.virtual_keycode);
                    if (input.state == ElementState::Pressed) {
                        match input.virtual_keycode {
                            None => {}
                            Some(val) => {
                                match val {
                                    VirtualKeyCode::F5 => {
                                        //NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::A => {
                                        //camera.position = Vec3::new(1.0, 0.0, 0.0);
                                        //NEED_UPDATE = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }

                    true
                }
                _ => { false }
            }
        },

        _ => { false }
    }
}