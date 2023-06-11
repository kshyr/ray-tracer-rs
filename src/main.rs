mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(&ray, 0.0, std::f32::MAX, &mut rec) {
        Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        ) * 0.5
    } else {
        let unit_direction = Vec3::unit_vector(ray.B);
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let byte_float: f32 = 255.999;
    let image_width = 1440;
    let image_height = 720;
    let max_value = 255;

    println!("P3\n{} {}\n{}", image_width, image_height, max_value);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let world = HittableList::new(list);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u: f32 = (i as f32) / (image_width as f32);
            let v: f32 = (j as f32) / (image_height as f32);
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let col: Vec3 = color(&r, &world);

            let ir = (byte_float * col.r()) as i32;
            let ig = (byte_float * col.g()) as i32;
            let ib = (byte_float * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
