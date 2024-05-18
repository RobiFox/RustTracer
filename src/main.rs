mod vec3;
mod libs;
mod ray;
mod hittable;
mod camera;
mod material;

use lazy_static::lazy_static;
use pixel_canvas::{Canvas, Color};
use pixel_canvas::canvas::CanvasInfo;
use pixel_canvas::input::{Event, MouseState, WindowEvent};
use pixel_canvas::input::glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere};
use crate::material::Material;
use crate::vec3::{Point3, Vec3};

const IMAGE_WIDTH: u32 = 350 * 2;
const IMAGE_HEIGHT: u32 = 250 * 2;

static mut VEC3_DEFAULT: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
static mut CAMERA: Camera = Camera {
    position: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    forward: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    world_up: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    u: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    v: Vec3 {x: 0.0, y: 0.0, z: 0.0},

    focal_length: 1.0,
    pixel00_loc: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    camera_center: Point3 {x: 0.0, y: 0.0, z: 0.0},
    pixel_delta_u: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    pixel_delta_v: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    pixel_sample_scale: 1.0,
    max_bounces: 1,
    samples_per_pixel: 1,
    vfov: 1.0,
};
static mut NEED_UPDATE: bool = true;

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

    //let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 1.0, 4, 255, 90.0);
    unsafe { CAMERA = Camera::new(Vec3::new(0.0, 0.0, 0.0), 1.0, 8, 1, 90.0); }
    let mut pixels = vec![];

    let canvas = Canvas::new(IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
        .title("Ray Trace")
        .state(MouseState::new())
        .input(handle_input);
    canvas.render(move |mouse, image| {
        unsafe {
            if !NEED_UPDATE {
                return;
            }
            pixels = CAMERA.render(&world);
            NEED_UPDATE = false;
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
                                    VirtualKeyCode::F5 => unsafe {
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::A => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(0.1, 0.0, 0.0);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::D => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(-0.1, 0.0, 0.0);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::W => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(0.0, 0.0, -0.1);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::S => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(0.0, 0.0, 0.1);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::E => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(0.0,  0.1, 0.0);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::Q => unsafe {
                                        CAMERA.position = CAMERA.position + Vec3::new(0.0, -0.1,0.0);
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::I => unsafe {
                                        CAMERA.forward = CAMERA.forward.rotate(0.25, 0.0).normalize();
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::K => unsafe {
                                        CAMERA.forward = CAMERA.forward.rotate(-0.25, 0.0).normalize();
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::J => unsafe {
                                        CAMERA.forward = CAMERA.forward.rotate(0.0, -0.25).normalize();
                                        NEED_UPDATE = true;
                                    },
                                    VirtualKeyCode::L => unsafe {
                                        CAMERA.forward = CAMERA.forward.rotate(0.0, 0.25).normalize();
                                        NEED_UPDATE = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }

                    true
                },
                _ => { false }
            }
        },

        _ => { false }
    }
}