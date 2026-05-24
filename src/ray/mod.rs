use crate::vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec3::Vec3,
    pub direction: vec3::Vec3
}

impl Ray {
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn empty() -> Ray {
        Ray {
            origin: vec3::Vec3::empty(),
            direction: vec3::Vec3::empty()
        }
    }
    /*
     * Returns vector at t distance along ray.
     */
    pub fn point_at_param(&self, t: f64) -> vec3::Vec3 {
        self.origin + t * self.direction
    }
}
