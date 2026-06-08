use crate::path_tracer::{vec3, materials};
use std::fmt;

#[repr(C)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: vec3::Vec3,
    pub surface_normal: vec3::Vec3,
    pub material: materials::Material,
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t: {}, p: {}, surf_norm: {}", self.t, self.p, self.surface_normal)
    }
}

impl HitRecord {
    pub fn empty() -> Self {
        Self {
            t: f64::MIN,
            p: vec3::Vec3::empty(),
            surface_normal: vec3::Vec3::empty(),
            material: materials::Material::new_none()
        }
    }
}
