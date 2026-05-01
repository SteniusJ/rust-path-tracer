use crate::{ray, vec3, materials};
use std::fmt;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    pub material: &'static dyn materials::Material,
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t: {}, p: {}, surf_norm: {}", self.t, self.p, self.surface_normal)
    }
}

impl HitRecord {
    pub fn empty(material: &'static dyn materials::Material) -> Self {
        Self {
            t: f64::MIN,
            p: vec3::Vec3::empty(),
            surface_normal: vec3::Vec3::empty(),
            material,
        }
    }
}

/*
 * Hitable trait. Implement for geometry that should be hitable.
 */
pub trait Hitable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
