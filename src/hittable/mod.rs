use crate::{ray, vec3, materials};
use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    pub material: Arc<dyn materials::Material>,
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t: {}, p: {}, surf_norm: {}", self.t, self.p, self.surface_normal)
    }
}

impl HitRecord {
    pub fn empty(material: Arc<dyn materials::Material>) -> Self {
        Self {
            t: f64::MIN,
            p: vec3::Vec3::empty(),
            surface_normal: vec3::Vec3::empty(),
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
