extern crate core;

use crate::ray::Ray;
use crate::vector::{dot, unit_vector, Vector3};

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

            let pixel_color = ray_color(ray);
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

fn ray_color(ray: Ray) -> Vector3 {
    let t = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = unit_vector(&(ray.at(t) - Vector3::new(0.0, 0.0, -1.0)));
        0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        let unit_direction = unit_vector(&ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Vector3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - &ray.origin;
    let a = ray.direction.length_squared();
    let h = dot(&ray.direction, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
