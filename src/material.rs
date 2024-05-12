use num_traits::Pow;
use rand::Rng;
use crate::hittable::HitRecord;
use crate::libs::random_unit_vector;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f64 },
    Dialectric { albedo: Vec3, refraction_index: f64 }
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
            Material::Metal { albedo, fuzziness } => {
                let reflect_dir = ray_in.direction.reflect(&hit_record.normal).normalize() + *fuzziness * random_unit_vector();
                *scattered = Ray::new(hit_record.point, reflect_dir);
                *attenuation = *albedo;
                scattered.direction.dot(&hit_record.normal) > 0.0
            }
            Material::Dialectric { albedo, refraction_index } => {
                *attenuation = *albedo;
                let ri = if hit_record.front_face { 1.0/refraction_index } else { *refraction_index };
                let unit_direction = ray_in.direction.normalize();

                let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let mut direction = Vec3::new(0.0, 0.0, 0.0);

                if cannot_refract || Self::reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0) {
                    direction = unit_direction.reflect(&hit_record.normal);
                } else {
                    direction = unit_direction.refract(&hit_record.normal, ri);
                }

                *scattered = Ray::new(hit_record.point, direction);
                true
            }
        }
    }

    fn reflectance(cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0 - cosine).pow(5)
    }
}