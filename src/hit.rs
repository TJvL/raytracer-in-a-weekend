use crate::ray::Ray;
use crate::vector::Vector3;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(point: Vector3, normal: Vector3, t: f64) -> Self {
        Self { point, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
