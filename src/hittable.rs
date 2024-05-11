use crate::material::{EmptyMaterial, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>
}

impl<'a> HitRecord<'a> {
    pub fn empty() -> HitRecord<'a> {
        HitRecord {
            t: 0.0,
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Point3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: &Box::new(EmptyMaterial {}),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center, radius, material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        //let b = -2.0 * ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a * c;

        if discriminant < 0.0 {
            return false
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if root <= ray_t_min || ray_t_max <= root {
            root = (h + sqrt_d) / a;
            if root <= ray_t_min || ray_t_max <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        //hit_record.material = self.material;
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        return true
    }
}

pub struct HittableList {
    pub vec: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record: HitRecord = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t_max;

        for object in &self.vec {
            if object.hit(ray, ray_t_min, ray_t_max, &mut temp_hit_record) {
                hit_anything = true;
                if temp_hit_record.t < closest_so_far {
                    closest_so_far = temp_hit_record.t;
                    *hit_record = temp_hit_record;
                }
            }
        }

        hit_anything
    }
}