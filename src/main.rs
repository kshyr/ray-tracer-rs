mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{scatter, Material};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&ray, 0.001, std::f32::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&ray.B);
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            break;
        }
    }

    p
}

fn main() {
    let byte_float: f32 = 255.999;
    let image_width = 1000;
    let image_height = 500;
    let samples = 64;
    let max_value = 255;

    println!("P3\n{} {}\n{}", image_width, image_height, max_value);

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.3, 0.3),
        },
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(0.6, -0.2, -0.4),
        0.3,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.6,
        },
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -0.3, -0.5),
        0.2,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            fuzz: 0.2,
        },
    )));

    let world = HittableList::new(list);

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut col = Vec3::default();

            for _ in 0..samples {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (image_width as f32);
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (image_height as f32);

                let r: Ray = camera.get_ray(u, v);

                col = col + color(&r, &world, 0);
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (byte_float * col.r()) as i32;
            let ig = (byte_float * col.g()) as i32;
            let ib = (byte_float * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
