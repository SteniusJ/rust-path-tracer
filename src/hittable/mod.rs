use crate::{ray, vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    //pub material: &Material
}

pub trait Hittable {
    fn hit(ray: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord);
}
