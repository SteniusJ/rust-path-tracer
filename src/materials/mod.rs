use crate::{ray, vec3, hittable};

pub trait Material {
    fn scatter(&self, ray: &ray::Ray, hit_record: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool;
}

/*
 * Materials
 */

pub struct Lambertian {
    pub albedo: vec3::Vec3
}

impl Lambertian {
    pub fn new(albedo: vec3::Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &ray::Ray, rec: &hittable::HitRecord, attentuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool {
        let target = rec.p + rec.surface_normal /* + random() */;
        *scattered = ray::Ray::new(rec.p, target - rec.p);
        *attentuation = self.albedo;
        true
    }
}
