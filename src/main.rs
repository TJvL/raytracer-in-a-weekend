extern crate core;

use std::f64::consts::PI;
use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::{unit_vector, Vector3};

mod hit;
mod ray;
mod sphere;
mod vector;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = if image_width / aspect_ratio < 1.0 {
        1.0
    } else {
        image_width / aspect_ratio
    };
    
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_center = Vector3::zero();

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / image_width;
    let pixel_delta_v = &viewport_v / image_height;

    let viewport_upper_left = &camera_center
        - Vector3::new(0.0, 0.0, focal_length)
        - &viewport_u / 2.0
        - viewport_v / 2.0;
    let first_pixel_coordinate = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for y in 0..image_height as usize {
        let h = y as f64;
        eprint!("\rScan lines remaining: {} ", y);
        for x in 0..image_width as usize {
            let w = x as f64;
            let pixel_center =
                &first_pixel_coordinate + (w * &pixel_delta_u) + (h * &pixel_delta_v);
            let ray_direction = pixel_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);

            let pixel_color = ray_color(ray, &world);
            write_color(pixel_color);
        }
    }

    eprintln!("\rDone.                 \n");
}

fn write_color(color: Vector3) {
    let r: u8 = (color.x * 255.999) as u8;
    let g: u8 = (color.y * 255.999) as u8;
    let b: u8 = (color.z * 255.999) as u8;

    println!("{} {} {}\n", r, g, b);
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Vector3 {
    if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
        0.5 * (hit.normal + Vector3::one())
    } else {
        let unit_direction = unit_vector(&ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::one() + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
