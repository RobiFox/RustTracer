use crate::hittable::HitRecord;
use crate::libs::random_unit_vector;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(self, ray_in: &Ray, hit_record: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct EmptyMaterial {}
impl Material for EmptyMaterial {
    fn scatter(self, ray_in: &Ray, hit_record: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        panic!("oops!")
    }
}

pub struct Lambertian {
    pub(crate) albedo: Vec3,
}
impl Material for Lambertian {
    fn scatter(self, ray_in: &Ray, hit_record: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() { scatter_direction = hit_record.normal }
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
}
impl Material for Metal {
    fn scatter(self, ray_in: &Ray, hit_record: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let scatter_direction = ray_in.direction.reflect(hit_record.normal);
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}