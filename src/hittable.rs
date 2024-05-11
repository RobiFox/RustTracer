use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Point3::new(0.0, 0.0, 0.0),
            front_face: false
        }
    }

    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
        self
    }
}

pub trait Hittable : Sync {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center, radius
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
        let outward_normal = (hit_record.point - self.center) / self.radius;
        *hit_record = hit_record.set_face_normal(ray, outward_normal);

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
                closest_so_far = temp_hit_record.t;
                *hit_record = temp_hit_record;
            }
        }

        hit_anything
    }
}