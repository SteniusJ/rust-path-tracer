use crate::{ray, vec3, hittable, util};
use rand::rngs::SmallRng;

pub trait Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool;
}

pub fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(v: &vec3::Vec3, n: &vec3::Vec3, ni_over_nt: &f64, refracted: &mut vec3::Vec3) -> bool {
    let uv = v.to_normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = *ni_over_nt * (uv - *n * dt) - *n * f64::sqrt(discriminant);
        return true;
    } else {
        return false;
    }
}

pub fn schlick(cosine: &f64, refraction_index: &f64) -> f64 {
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

#[derive(Clone)]
pub struct Metal {
    pub albedo: vec3::Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: vec3::Vec3, fuzz: f64) -> Metal {
        let fuzz = {
            if fuzz < 1.0 {
                fuzz
            } else {
                1.0
            }
        };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool {
        let reflected = reflect(&r_in.direction.to_normalized(), &rec.surface_normal);
        *scattered = ray::Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        *attentuation = self.albedo;
        return scattered.direction.dot(&rec.surface_normal) > 0.0;
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &ray::Ray, rec: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool {
        let outward_normal: vec3::Vec3;
        let ni_over_nt: f64;
        let reflect_prob: f64;
        let cosine: f64;
        let reflected = reflect(&r_in.direction, &rec.surface_normal);
        let mut refracted = vec3::Vec3::empty();

        *attentuation = vec3::Vec3::new(1.0, 1.0, 1.0);

        if r_in.direction.dot(&rec.surface_normal) > 0.0 {
            outward_normal = -rec.surface_normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * r_in.direction.dot(&rec.surface_normal) / r_in.direction.len();
        } else {
            outward_normal = rec.surface_normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -r_in.direction.dot(&rec.surface_normal) / r_in.direction.len();
        }

        if refract(&r_in.direction, &outward_normal, &ni_over_nt, &mut refracted) {
            reflect_prob = schlick(&cosine, &self.refraction_index);
        } else {
            reflect_prob = 1.0;
        }

        if util::randf(rng) < reflect_prob {
            *scattered = ray::Ray::new(rec.p, reflected);
        } else {
            *scattered = ray::Ray::new(rec.p, refracted);
        }

        true
    }
}

#[derive(Clone)]
pub struct Normal {
}

impl Normal {
    pub fn new() -> Normal {
        Normal {  }
    }
}

impl Material for Normal {
    fn scatter(&self, _r_in: &ray::Ray, rec: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, rng: &mut SmallRng) -> bool {
        let target = rec.p + rec.surface_normal + random_in_unit_sphere(rng);
        *scattered = ray::Ray::new(rec.p, target - rec.p);
        let mut normal_color = rec.surface_normal.to_normalized();
        normal_color.into_positive();
        *attentuation = normal_color;
        true
    }
}
