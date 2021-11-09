mod camera;
mod hit;
mod mat;
mod math;
mod ray;

use std::f64::INFINITY;

#[allow(unused_imports)]
use cgmath::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;

use camera::Camera;
use hit::list::HittableList;
use hit::sphere::Sphere;
use hit::Hittable;
use image::RgbImage;
use mat::lambertian::Lambertian;
use mat::metal::Metal;
use mat::Material;
pub use math::Vec3;
use ray::Ray;

use crate::mat::dielectric::Dielectric;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const ASPECT_RATIO: f64 = 1.777_777_777_777_777_7;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_RAY_DEPTH: i32 = 50;

fn main() {
    let mut pixels = ndarray::Array2::<Vec3>::zeros((IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize));

    let ground = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let centre = Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    };
    let left = Dielectric { ir: 1.5 };
    let right = Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let mut world = HittableList(Vec::new());

    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: Material::Dielectric(left),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric(left),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(&right),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian(&centre),
    }));
    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::new(0.0, -100.5, -1.0),
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
                let mut colour = Vec3::zero();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let vals: [f64; 2] = rand::thread_rng().gen();
                    let u = (row_number as f64 + vals[0]) / (IMAGE_WIDTH as f64 - 1f64);
                    let v = (col_number as f64 + vals[1]) / (IMAGE_HEIGHT as f64 - 1f64);
                    let r = camera.get_ray(u, v);

                    colour += ray_colour(&r, &world, MAX_RAY_DEPTH);
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

fn ray_colour(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth == 0 {
        Vec3::zero()
    } else {
        match world.hit(ray, 0.001, INFINITY) {
            Some(hit) => {
                let (attenuation, scattered) = hit.material.scatter(ray, &hit);
                let r = ray_colour(&scattered, world, depth - 1);
                attenuation * r
            }
            None => {
                let unit_dir = ray.direction.unit_vec();
                let t = 0.5 * (unit_dir.y + 1.0);
                (1.0 - t) * Vec3::one() + t * Vec3::new(0.4, 0.7, 1.0)
            }
        }
    }
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
