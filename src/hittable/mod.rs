use crate::{ray, vec3, materials};

pub struct HitRecord<'a, T: materials::Material> {
    pub t: f64,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    pub material: &'a T,
}

pub trait Hittable<'a, T> where T: materials::Material {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'a, T>) -> bool;
}
