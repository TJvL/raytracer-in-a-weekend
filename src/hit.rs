use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{dot, Vector3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3, t: f64, outward_normal: Vector3, ray_direction: &Vector3) -> Self {
        let is_front_face = dot(ray_direction, &outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            t,
            is_front_face,
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                closest_hit_record = Some(hit);
            }
        }

        closest_hit_record
    }
}
