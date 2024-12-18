use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{dot, Vector3};

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = &self.center - &ray.origin;
        let a = ray.direction.length_squared();
        let h = dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_d = discriminant.sqrt();

            let mut root = (h - sqrt_d) / a;

            if !ray_t.surrounds(root) {
                root = (h + sqrt_d) / a;
                if !ray_t.surrounds(root) {
                    return None;
                }
            }

            let point = ray.at(root);
            let outward_normal = (&point - &self.center) / self.radius;

            Some(HitRecord::new(
                point,
                root,
                outward_normal,
                &ray.direction,
                &*self.material,
            ))
        }
    }
}
