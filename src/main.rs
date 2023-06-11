mod hittable;
mod ray;
mod sphere;
mod vec3;

use hittable::HitRecord;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vec3::unit_vector(ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(ray.B);
    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.A - center;
    let a = Vec3::dot(&ray.B, &ray.B);
    let b = Vec3::dot(&oc, &ray.B) * 2.0;
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / 2.0 * a
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

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u: f32 = (i as f32) / (image_width as f32);
            let v: f32 = (j as f32) / (image_height as f32);
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(&r);

            let ir = (byte_float * col.r()) as i32;
            let ig = (byte_float * col.g()) as i32;
            let ib = (byte_float * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}
