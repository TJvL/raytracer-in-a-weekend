use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector::{random_unit_vector, reflect, Vector3};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuated: Vector3,
}

impl ScatterResult {
    pub fn new(scattered: Ray, attenuated: Vector3) -> Self {
        Self {
            scattered,
            attenuated,
        }
    }
}

pub trait Material {
    fn scatter(&self, hit_record: HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_record: HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = &hit_record.normal + random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal.clone();
        };

        let scattered_ray = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo.clone();
        Some(ScatterResult::new(scattered_ray, attenuation))
    }
}

pub struct Metal {
    albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&hit_record.normal, &self.albedo);
        let scattered_ray = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo.clone();
        Some(ScatterResult::new(scattered_ray, attenuation))
    }
}
