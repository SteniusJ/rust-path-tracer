#![allow(dead_code)]

use crate::path_tracer::{ray, vec3, hitable, util};

use cuda_device::device;

#[derive(Clone, Copy)]
pub enum MaterialID {
    Lambertian,
    Metal,
    Dielectric,
    Normal,
    None
}

/*
 * Material struct, has to be used instead of enum with values since cuda oxide doesn't support
 * those.
 */
#[derive(Clone, Copy)]
pub struct Material {
    id: MaterialID,
    albedo: vec3::Vec3,
    fuzz: f64,
    refraction_index: f64
}

impl Material {
    pub fn new_lambertian(albedo: vec3::Vec3) -> Material {
        Material {
            id: MaterialID::Lambertian,
            albedo,
            fuzz: 0.0,
            refraction_index: 0.0
        }
    }
    pub fn new_dielectric(refraction_index: f64) -> Material {
        Material {
            id: MaterialID::Dielectric,
            albedo: vec3::Vec3::empty(),
            fuzz: 0.0,
            refraction_index
        }
    }
    pub fn new_normal(border_color: vec3::Vec3, border_treshold: f64) -> Material {
        Material {
            id: MaterialID::Normal,
            albedo: border_color,
            fuzz: border_treshold,
            refraction_index: 0.0
        }
    }
    pub fn new_metal(albedo: vec3::Vec3, fuzz: f64) -> Material {
        let fuzz = {
            if fuzz < 1.0 {
                fuzz
            } else {
                1.0
            }
        };
        Material {
            id: MaterialID::Metal,
            albedo,
            fuzz,
            refraction_index: 0.0
        }
    }
    pub fn new_none() -> Material {
        Material {
            id: MaterialID::None,
            albedo: vec3::Vec3::empty(),
            fuzz: 0.0,
            refraction_index: 0.0
        }
    }
}

#[device]
pub fn scatter(ray: &ray::Ray, hit_record: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let (id, albedo, fuzz, refraction_index) = (
        hit_record.material.id,
        hit_record.material.albedo,
        hit_record.material.fuzz,
        hit_record.material.refraction_index
        );

    match id {
        MaterialID::Lambertian => lambertian_scatter(albedo, hit_record, attentuation, scattered, seed),
        MaterialID::Metal => metal_scatter(albedo, fuzz, ray, hit_record, attentuation, scattered, seed),
        MaterialID::Dielectric => dielectric_scatter(refraction_index, ray, hit_record, attentuation, scattered, seed),
        MaterialID::Normal => normal_scatter(hit_record, attentuation, albedo, fuzz),
        MaterialID::None => false,
    }
}

pub fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(v: &vec3::Vec3, n: &vec3::Vec3, ni_over_nt: &f64, refracted: &mut vec3::Vec3) -> bool {
    let uv = v.normalized();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = *ni_over_nt * (uv - *n * dt) - *n * util::sqrt_f64(discriminant);
        true
    } else {
        false
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
    let reflected = reflect(&r_in.direction.normalized(), &rec.surface_normal);
    *scattered = ray::Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere(seed));
    *attentuation = albedo;
    scattered.direction.dot(&rec.surface_normal) > 0.0
}

/*
 * Dielectric material.
 * Glass like material. Accurate glass like material with refraction_index = 1.5.
 */
fn dielectric_scatter(refraction_index: f64, r_in: &ray::Ray, rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray, seed: &mut u32) -> bool {
    let reflected = reflect(&r_in.direction, &rec.surface_normal);
    let mut refracted = vec3::Vec3::empty();
    *attentuation = vec3::Vec3::new(1.0, 1.0, 1.0);

    let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(&rec.surface_normal) > 0.0 {
        (
            -rec.surface_normal,
            refraction_index,
            refraction_index * r_in.direction.dot(&rec.surface_normal) / r_in.direction.len()
            )
    } else {
        (
            rec.surface_normal,
            1.0 / refraction_index,
            -r_in.direction.dot(&rec.surface_normal) / r_in.direction.len()
            )
    };

    let reflect_prob = if refract(&r_in.direction, &outward_normal, &ni_over_nt, &mut refracted) {
        schlick(&cosine, &refraction_index)
    } else {
        1.0
    };

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
 * Also visualises triangle borders.
 */
fn normal_scatter(rec: &hitable::HitRecord, attentuation: &mut vec3::Vec3, border_color: vec3::Vec3, border_treshold: f64) -> bool {
    let normal_color = rec.surface_normal.normalized();
    normal_color.to_positive();

    if rec.uv > border_treshold && rec.uv < 1.0 {
        *attentuation = border_color;
        return false;
    }

    *attentuation = normal_color;
    false
}
