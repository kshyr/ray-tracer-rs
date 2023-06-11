use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32, _rec: &mut HitRecord) -> bool {
        false
    }
}
