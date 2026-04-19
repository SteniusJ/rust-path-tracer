use crate::{vec3, ray, util};
use std::f64::consts::PI;

struct Camera {
    pub origin: vec3::Vec3,
    pub lower_left_corner: vec3::Vec3,
    pub horizontal: vec3::Vec3,
    pub vertical: vec3::Vec3,
    pub u: vec3::Vec3,
    pub v: vec3::Vec3,
    pub w: vec3::Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: vec3::Vec3, look_at: vec3::Vec3, v_up: vec3::Vec3, v_fov: f64, aspect: f64, aperature: f64, focus_dist: f64) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).to_normalized();
        let u = v_up.cross(&w).to_normalized();
        let v = w.cross(&u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperature / 2.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        ray::Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}

fn random_in_unit_disk() -> vec3::Vec3 {
    let mut p: vec3::Vec3 = vec3::Vec3::empty();

    while p.dot(&p) >= 1.0 {
        p = 2.0 * vec3::Vec3::new(util::randf(), util::randf(), 0.0) - vec3::Vec3::new(1.0, 1.0, 0.0);
    }
    p
}
