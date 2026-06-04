use crate::path_tracer::{ray, vec3, hitable, util};

use cuda_device::device;

/* 
 * Update materials to use enum instead of traits since GPU code can't handle those 
 */
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: vec3::Vec3 },
    Metal { albedo: vec3::Vec3, fuzz: f64 },
    Dielectric { refraction_index: f64 },
    Normal
}

impl Material {
    pub fn new_lambertian(albedo: vec3::Vec3) -> Material {
        Material::Lambertian { albedo }
    }
    pub fn new_dielectric(refraction_index: f64) -> Material {
        Material::Dielectric { refraction_index }
    }
    pub fn new_normal() -> Material {
        Material::Normal
    }
    pub fn new_metal(albedo: vec3::Vec3, fuzz: f64) -> Material {
        let fuzz = {
            if fuzz < 1.0 {
                fuzz
            } else {
                1.0
            }
        };
        Material::Metal { albedo, fuzz }
    }
}

/* 
 * CPU implementation of enum based scatter function
 */
#[device]
pub fn scatter(ray: &ray::Ray, hit_record: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    match hit_record.material {
        Material::Lambertian { albedo } => lambertian_scatter(albedo, hit_record, attentuation, scattered, seed),
        Material::Metal { albedo, fuzz } => metal_scatter(albedo, fuzz, ray, hit_record, attentuation, scattered, seed),
        Material::Dielectric { refraction_index } => dielectric_scatter(refraction_index, ray, hit_record, attentuation, scattered, seed),
        Material::Normal => normal_scatter(hit_record, attentuation, scattered, seed)
    }
}

pub fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(v: &vec3::Vec3, n: &vec3::Vec3, ni_over_nt: &f64, refracted: &mut vec3::Vec3) -> bool {
    let uv = v.to_normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = *ni_over_nt * (uv - *n * dt) - *n * util::sqrt_f64(discriminant);
        return true;
    } else {
        return false;
    }
}

pub fn schlick(cosine: &f64, refraction_index: &f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0; // reassinging to avoid unnecessary mutables

    r0 + (1.0 - r0) * util::powi_f64(1.0 - cosine, 5)
}

pub fn random_in_unit_sphere(seed: &mut u32) -> vec3::Vec3 {
    let mut p = 2.0 * vec3::Vec3::new(util::randf(seed), util::randf(seed), util::randf(seed)) - vec3::Vec3::new(1.0, 1.0, 1.0);
    
    while p.sqrt_len() >= 1.0 {
        p = 2.0 * vec3::Vec3::new(util::randf(seed), util::randf(seed), util::randf(seed)) - vec3::Vec3::new(1.0, 1.0, 1.0);
    }

    p
}

/*
 * Materials
 */

/*
 * Lambertian material
 * Opaque Matte material
 */
fn lambertian_scatter(albedo: vec3::Vec3, rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let target = rec.p + rec.surface_normal + random_in_unit_sphere(seed);
    *scattered = ray::Ray::new(rec.p, target - rec.p);
    *attentuation = albedo;
    true
}

/*
 * Metal material.
 * Metallic. Fuzz is surface roughness, higher fuzz == rougher, lower fuzz == more polished.
 * 0.0 fuzz == mirror like perfect reflection.
 */
fn metal_scatter(albedo: vec3::Vec3, fuzz: f64, r_in: &ray::Ray, rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let reflected = reflect(&r_in.direction.to_normalized(), &rec.surface_normal);
    *scattered = ray::Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere(seed));
    *attentuation = albedo;
    return scattered.direction.dot(&rec.surface_normal) > 0.0;
}

/*
 * Dielectric material.
 * Glass like material. Accurate glass like material with refraction_index = 1.5.
 */
fn dielectric_scatter(refraction_index: f64, r_in: &ray::Ray, rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let outward_normal: vec3::Vec3;
    let ni_over_nt: f64;
    let reflect_prob: f64;
    let cosine: f64;
    let reflected = reflect(&r_in.direction, &rec.surface_normal);
    let mut refracted = vec3::Vec3::empty();

    *attentuation = vec3::Vec3::new(1.0, 1.0, 1.0);

    if r_in.direction.dot(&rec.surface_normal) > 0.0 {
        outward_normal = -rec.surface_normal;
        ni_over_nt = refraction_index;
        cosine = refraction_index * r_in.direction.dot(&rec.surface_normal) / r_in.direction.len();
    } else {
        outward_normal = rec.surface_normal;
        ni_over_nt = 1.0 / refraction_index;
        cosine = -r_in.direction.dot(&rec.surface_normal) / r_in.direction.len();
    }

    if refract(&r_in.direction, &outward_normal, &ni_over_nt, &mut refracted) {
        reflect_prob = schlick(&cosine, &refraction_index);
    } else {
        reflect_prob = 1.0;
    }

    if util::randf(seed) < reflect_prob {
        *scattered = ray::Ray::new(rec.p, reflected);
    } else {
        *scattered = ray::Ray::new(rec.p, refracted);
    }

    true
}

/*
 * Normal material.
 * Debug material for visualization of surface normals.
 */
fn normal_scatter(rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let target = rec.p + rec.surface_normal + random_in_unit_sphere(seed);
    *scattered = ray::Ray::new(rec.p, target - rec.p);
    let mut normal_color = rec.surface_normal.to_normalized();
    normal_color.into_positive();
    *attentuation = normal_color;
    true
}
