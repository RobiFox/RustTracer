use crate::hittable::HitRecord;
use crate::libs::random_unit_vector;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal
                }
                *scattered = Ray::new(hit_record.point, scatter_direction);
                *attenuation = *albedo;
                true
            }
            Material::Metal { albedo } => {
                let reflect_dir = ray_in.direction.reflect(hit_record.normal);
                *scattered = Ray::new(hit_record.point, reflect_dir);
                *attenuation = *albedo;
                true
            }
        }
    }
}