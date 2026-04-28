use crate::{ray, vec3, hittable, util};
use rand::rngs::SmallRng;

pub trait Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool;
}

pub fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(v: &vec3::Vec3, n: &vec3::Vec3, ni_over_nt: f64, refracted: &mut vec3::Vec3) -> bool {
    let uv = v.to_normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - *n * dt) - *n * f64::sqrt(discriminant);
        return true;
    } else {
        return false;
    }
}

pub fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0; // reassinging to avoid unnecessary mutables

    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}

pub fn random_in_unit_sphere(rng: &mut SmallRng) -> vec3::Vec3 {
    let mut p = 2.0 * vec3::Vec3::new(util::randf(rng), util::randf(rng), util::randf(rng)) - vec3::Vec3::new(1.0, 1.0, 1.0);
    
    while p.sqrt_len() >= 1.0 {
        p = 2.0 * vec3::Vec3::new(util::randf(rng), util::randf(rng), util::randf(rng)) - vec3::Vec3::new(1.0, 1.0, 1.0);
    }

    p
}

/*
 * Materials
 */

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: vec3::Vec3
}

impl Lambertian {
    pub fn new(albedo: vec3::Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &ray::Ray, rec: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool {
        let target = rec.p + rec.surface_normal + random_in_unit_sphere(rng);
        *scattered = ray::Ray::new(rec.p, target - rec.p);
        *attentuation = self.albedo;
        true
    }
}
