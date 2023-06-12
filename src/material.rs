use crate::{hittable::HitRecord, random_in_unit_sphere, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attentuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            *scattered = Ray::new(rec.p, target - rec.p);
            *attentuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let mut f = 1.0;
            if f < 1.0 {
                f = fuzz;
            }
            let reflected = reflect(&Vec3::unit_vector(&ray_in.B), &rec.normal);
            *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * fuzz);
            *attentuation = albedo;
            return Vec3::dot(&scattered.B, &rec.normal) > 0.0;
        }
        Material::Dielectric {} => todo!(),
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * Vec3::dot(v, n) * 2.0
}
