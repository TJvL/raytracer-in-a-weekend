use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector::{dot, Vector3};

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    fn new(center: Vector3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

            if root < t_min || t_max < root {
                root = (h + sqrt_d) / a;
                if root <= t_min || t_max <= root {
                    return None;
                }
            }

            let point = ray.at(root);
            let normal = (&point - &self.center) / self.radius;

            Some(HitRecord::new(point, normal, root))
        }
    }
}
