use crate::{ray, vec3};

pub struct HitRecord {
    t: f32,
    p: vec3::Vec3,
    surface_normal: vec3::Vec3,
    //material: &Material
}

pub trait Hittable {
    fn hit(ray: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord);
}
