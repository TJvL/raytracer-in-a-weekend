use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{unit_vector, Vector3};
use std::f64::consts::PI;

pub struct Camera {
    aspect_ratio: f64,
    image_width: f64,
    image_height: f64,
    center: Vector3,
    first_pixel_coordinate: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u16) -> Self {
        let image_width = image_width as f64;
        let image_height = if image_width / aspect_ratio < 1.0 {
            1.0
        } else {
            image_width / aspect_ratio
        };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width / image_height);
        let center = Vector3::zero();

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = &viewport_u / image_width;
        let pixel_delta_v = &viewport_v / image_height;

        let viewport_upper_left =
            &center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let first_pixel_coordinate = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            first_pixel_coordinate,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for y in 0..self.image_height as usize {
            let h = y as f64;
            eprint!("\rScan lines remaining: {} ", y);
            for x in 0..self.image_width as usize {
                let w = x as f64;
                let pixel_center = &self.first_pixel_coordinate
                    + (w * &self.pixel_delta_u)
                    + (h * &self.pixel_delta_v);
                let ray_direction = pixel_center - &self.center;
                let ray = Ray::new(self.center.clone(), ray_direction);

                let pixel_color = ray_color(ray, world);
                write_color(pixel_color);
            }
        }

        eprintln!("\rDone.                 \n");
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Vector3 {
    if let Some(hit) = world.hit(&ray, Interval::new(0.0, f64::INFINITY)) {
        0.5 * (hit.normal + Vector3::one())
    } else {
        let unit_direction = unit_vector(&ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::one() + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn write_color(color: Vector3) {
    let r: u8 = (color.x * 255.999) as u8;
    let g: u8 = (color.y * 255.999) as u8;
    let b: u8 = (color.z * 255.999) as u8;

    println!("{} {} {}\n", r, g, b);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
