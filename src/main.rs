use crate::camera::Camera;
use crate::hit::HittableList;
use crate::sphere::Sphere;
use crate::vector::Vector3;

mod camera;
mod hit;
mod interval;
mod ray;
mod sphere;
mod utility;
mod vector;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let samples_per_pixel: u8 = 100;
    let max_depth: usize = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(&world);
}
