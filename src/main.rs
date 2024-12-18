use crate::camera::Camera;
use crate::hit::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vector::Vector3;

mod camera;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vector;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Box::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Metal::new(Vector3::new(0.8, 0.8, 0.8)));
    let material_right = Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vector3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let samples_per_pixel: u8 = 100;
    let max_depth: usize = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(&world);
}
