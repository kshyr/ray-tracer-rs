use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.A - self.center;
        let a = Vec3::dot(&ray.B, &ray.B);
        let b = Vec3::dot(&oc, &ray.B);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }
        }
        None
    }
}
