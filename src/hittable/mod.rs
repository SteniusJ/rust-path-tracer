use crate::{ray, vec3, materials};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    pub material: &'a dyn materials::Material,
}

impl<'a> HitRecord<'a> {
    pub fn empty(material: &'a dyn materials::Material) -> Self {
        Self {
            t: f64::MIN,
            p: vec3::Vec3::empty(),
            surface_normal: vec3::Vec3::empty(),
            material,
        }
    }
}

pub trait Hittable<'a> {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool;
}
