use crate::vec3;

pub struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3
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
    fn point_at_param(&self, t: f32) -> vec3::Vec3 {
        self.origin + t * self.direction
    }
}
