use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility::{random_from_range, CLOSEST_TO_ZERO_TO_ONE_RANGE};
use crate::vector::{unit_vector, Vector3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: f64,
    image_height: f64,
    pixel_samples_scale: f64,
    max_depth: usize,
    samples_per_pixel: u8,
    center: Vector3,
    first_pixel_coordinate: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u16,
        samples_per_pixel: u8,
        max_depth: usize,
    ) -> Self {
        let image_width = image_width as f64;
        let image_height = if image_width / aspect_ratio < 1.0 {
            1.0
        } else {
            image_width / aspect_ratio
        };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

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
            pixel_samples_scale,
            samples_per_pixel,
            max_depth,
            center,
            first_pixel_coordinate,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for y in 0..self.image_height as usize {
            let u = y as f64;
            eprint!("\rScan lines remaining: {} ", y);
            for x in 0..self.image_width as usize {
                let v = x as f64;
                let mut pixel_color = Vector3::zero();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(u, v);
                    pixel_color += ray_color(ray, self.max_depth, world);
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }

        eprintln!("\rDone.                 \n");
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = sample_square();
        let pixel_sample = &self.first_pixel_coordinate
            + ((v + offset.x) * &self.pixel_delta_u)
            + ((u + offset.y) * &self.pixel_delta_v);
        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vector3 {
    let x = random_from_range(CLOSEST_TO_ZERO_TO_ONE_RANGE) - 0.5;
    let y = random_from_range(CLOSEST_TO_ZERO_TO_ONE_RANGE) - 0.5;
    let z = 0.0;

    Vector3::new(x, y, z)
}

fn ray_color(ray: Ray, max_depth: usize, world: &dyn Hittable) -> Vector3 {
    if max_depth == 0 {
        Vector3::zero()
    } else if let Some(hit_record) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
        if let Some(scatter_result) = hit_record.material.scatter(hit_record) {
            scatter_result.attenuated * ray_color(scatter_result.scattered, max_depth - 1, world)
        } else {
            Vector3::zero()
        }
    } else {
        let unit_direction = unit_vector(&ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::one() + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn write_color(color: Vector3) {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let intensity = Interval::new(0.0, 0.999);
    let r_byte: u8 = (256.0 * intensity.clamp(r)) as u8;
    let g_byte: u8 = (256.0 * intensity.clamp(g)) as u8;
    let b_byte: u8 = (256.0 * intensity.clamp(b)) as u8;

    println!("{} {} {}\n", r_byte, g_byte, b_byte);
}
