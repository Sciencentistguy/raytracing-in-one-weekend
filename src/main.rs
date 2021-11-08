mod camera;
mod colour;
mod hit;
mod mat;
mod math;
mod point;
mod ray;

use std::f64::INFINITY;

#[allow(unused_imports)]
use cgmath::prelude::*;
use ndarray::Axis;
use rayon::prelude::*;

use colour::Colour;
use hit::list::HittableList;
use hit::Hittable;
use image::RgbImage;
pub use math::Vec3;
use point::Point;
use ray::Ray;

use crate::camera::Camera;
use crate::hit::sphere::Sphere;
use crate::mat::lambertian::Lambertian;
use crate::mat::metal::Metal;
use crate::mat::Material;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const ASPECT_RATIO: f64 = 1.777_777_777_777_777_7;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_RAY_DEPTH: usize = 50;

fn main() {
    let mut pixels =
        ndarray::Array2::<Colour>::zeros((IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize));

    let ground = Lambertian {
        albedo: Colour(Vec3::new(0.8, 0.8, 0.0)),
    };
    let centre = Lambertian {
        albedo: Colour(Vec3::new(0.7, 0.3, 0.3)),
    };
    let left = Metal {
        albedo: Colour(Vec3::new(0.8, 0.8, 0.8)),
        fuzz: 0.3,
    };
    let right = Metal {
        albedo: Colour(Vec3::new(0.8, 0.6, 0.2)),
        fuzz: 1.0,
    };

    let mut world = HittableList(Vec::new());

    world.0.push(Hittable::Sphere(Sphere {
        centre: Point(Vec3::new(-1, 0, -1)),
        radius: 0.5,
        material: Material::Metal(&left),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Point(Vec3::new(1, 0, -1)),
        radius: 0.5,
        material: Material::Metal(&right),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Point(Vec3::new(0, 0, -1)),
        radius: 0.5,
        material: Material::Lambertian(&centre),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Point(Vec3::new(0.0, -100.5, -1.0)),
        radius: 100.0,
        material: Material::Lambertian(&ground),
    }));

    let camera = Camera::new();

    let start = std::time::Instant::now();

    pixels
        .outer_iter_mut()
        .into_par_iter()
        .enumerate()
        .for_each(|(row_number, mut row)| {
            for (col_number, pixel) in row.iter_mut().enumerate() {
                let mut colour = Colour(Vec3::zero());
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (row_number as f64 + rand_f64!()) / (IMAGE_WIDTH as f64 - 1f64);
                    let v = (col_number as f64 + rand_f64!()) / (IMAGE_HEIGHT as f64 - 1f64);
                    let r = camera.get_ray(u, v);

                    colour += ray_colour(&r, &world, 0);
                }

                *pixel = colour;
            }
        });

    println!("Took {:?}", start.elapsed());

    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pix = pixels.get((i as usize, j as usize)).unwrap();
            image.put_pixel(i, IMAGE_HEIGHT - 1 - j, pix.to_pixel(SAMPLES_PER_PIXEL));
        }
    }

    image.save("image.png").unwrap();
}

fn ray_colour(ray: &Ray, world: &HittableList, depth: usize) -> Colour {
    if depth > MAX_RAY_DEPTH {
        return Colour(Vec3::zero());
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            return Colour(attenuation.0 * ray_colour(&scattered, world, depth + 1).0);
        }
        return Colour(Vec3::zero());
        //let target = hit.p.0 + hit.normal + Vec3::random_unit_vector();
        //let target = hit.p.0 + Vec3::random_in_hemisphere(hit.normal);
        //return Colour(
        //0.5 * ray_colour(
        //&Ray {
        //origin: hit.p,
        //direction: target - hit.p.0,
        //},
        //world,
        //depth + 1,
        //)
        //.0,
        //);
    }

    let unit_dir = ray.direction.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);
    Colour((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.4, 0.7, 1.0))
}

#[macro_export]
macro_rules! rand_f64 {
    () => {{
        use rand::Rng;
        rand::thread_rng().gen_range(0.0..=1.0)
    }};
    ($rng:expr) => {{
        use rand::Rng;
        rand::thread_rng().gen_range($rng)
    }};
}
