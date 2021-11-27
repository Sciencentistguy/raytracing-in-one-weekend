mod camera;
mod hit;
mod mat;
mod math;
mod ray;

use std::f64;
use std::sync::Arc;

use cgmath::Deg;
use rand::prelude::*;

// .into_par_iter()  is often commented out for profiling, so this will scream
#[allow(unused_imports)]
use rayon::prelude::*;

pub use math::Vec3;

use camera::Camera;
use hit::list::HittableList;
use hit::sphere::Sphere;
use hit::Hittable;
use image::RgbImage;
use mat::dielectric::Dielectric;
use mat::lambertian::Lambertian;
use mat::metal::Metal;
use mat::Material;
use ray::Ray;

const IMAGE_WIDTH: u32 = 1800;
const IMAGE_HEIGHT: u32 = 1200;
const ASPECT_RATIO: f64 = 1.5;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_RAY_DEPTH: i32 = 50;

fn main() {
    let mut pixels = ndarray::Array2::<Vec3>::zeros((IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize));

    let world = random_scene();

    const CAMERA_POS: Vec3 = Vec3::newi(13, 2, 3);
    const CAMERA_TARGET: Vec3 = Vec3::newi(0, 0, 0);
    const DIST_TO_FOCUS: f64 = 10.0;
    const CAMERA_APERTURE: f64 = 0.1;

    let camera = Camera::new(
        CAMERA_POS,
        CAMERA_TARGET,
        Vec3::UNIT_UP,
        Deg(20.0),
        ASPECT_RATIO,
        CAMERA_APERTURE,
        DIST_TO_FOCUS,
    );

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
        match world.hit(ray, 0.001, f64::INFINITY) {
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

fn random_scene() -> HittableList {
    let mut world = HittableList(Vec::new());

    const GROUND_MATERIAL: Lambertian = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    const GLASS_MATERIAL: Dielectric = Dielectric { ir: 1.5 };
    const SPHERE_2_MATERIAL: Lambertian = Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    const SPHERE_3_MATERIAL: Metal = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::newi(0, -1000, 0),
        radius: 1000.0,
        material: Material::Lambertian(Arc::new(GROUND_MATERIAL)),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rand_f64!();
            let centre = Vec3::new(a + 0.9 * rand_f64!(), 0.2, b + 0.9 * rand_f64!());
            let radius = 0.2;

            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Lambertian { albedo };
                    world.0.push(Hittable::Sphere(Sphere {
                        centre,
                        radius,
                        material: Material::Lambertian(Arc::new(sphere_material)),
                    }))
                    //diffuse
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_with_range(0.5, 1.0);
                    let fuzz = rand_f64!(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.0.push(Hittable::Sphere(Sphere {
                        centre,
                        radius,
                        material: Material::Metal(Arc::new(sphere_material)),
                    }))
                } else {
                    //glass
                    world.0.push(Hittable::Sphere(Sphere {
                        centre,
                        radius,
                        material: Material::Dielectric(GLASS_MATERIAL),
                    }))
                }
            }
        }
    }

    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::newi(0, 1, 0),
        radius: 1.0,
        material: Material::Dielectric(GLASS_MATERIAL),
    }));

    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::newi(-4, 1, 0),
        radius: 1.0,
        material: Material::Lambertian(Arc::new(SPHERE_2_MATERIAL)),
    }));

    world.0.push(Hittable::Sphere(Sphere {
        centre: Vec3::newi(4, 1, 0),
        radius: 1.0,
        material: Material::Metal(Arc::new(SPHERE_3_MATERIAL)),
    }));

    world
}
