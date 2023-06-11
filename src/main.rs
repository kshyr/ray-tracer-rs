mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(ray.B);
    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let byte_float: f32 = 255.999;
    let image_width = 1000;
    let image_height = 1000;
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
