use crate::{vec3, ray, util};
use std::f64::consts::PI;

#[repr(C)]
#[derive(Debug)]
pub struct Camera {
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
    /*
     * Camera constructor
     */
    pub fn new(look_from: vec3::Vec3, look_at: vec3::Vec3, v_up: vec3::Vec3, v_fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
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
            lens_radius: aperture / 2.0,
        }
    }
    /*
     * Returns ray pointing from camera origin to given screen space coordinates.
     * Adds slight randomness to ray position.
     */
    pub fn get_ray(&self, s: f64, t: f64, seed: &mut u32) -> ray::Ray {
        let rd = self.lens_radius * random_in_unit_disk(seed);
        let offset = self.u * rd.x + self.v * rd.y;

        ray::Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
    /*
     * For some reason current CUDA Oxide doesn't support structs with multiple
     * fields as arguments in a kernel. Because of this the camera has to be converted into
     * this convoluted tuple.
     */
    pub fn into_gpu_arg(self) -> (
        (f64, f64, f64),
        (f64, f64, f64),
        (f64, f64, f64),
        (f64, f64, f64),
        (f64, f64, f64),
        (f64, f64, f64),
        (f64, f64, f64),
        f64
        ) {
        (
            (self.origin.x, self.origin.y, self.origin.z),
            (self.lower_left_corner.x, self.lower_left_corner.y, self.lower_left_corner.z),
            (self.horizontal.x, self.horizontal.y, self.horizontal.z),
            (self.vertical.x, self.vertical.y, self.vertical.z),
            (self.u.x, self.u.y, self.u.z),
            (self.v.x, self.v.y, self.v.z),
            (self.w.x, self.w.y, self.w.z),
            self.lens_radius
        )
    }
    /*
     * Converts tuple back to camera
     */
    pub fn from_gpu_arg(
        arg: (
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            f64
        )
        ) -> Self {
        Self {
            origin: vec3::Vec3::new(arg.0.0, arg.0.1, arg.0.2),
            lower_left_corner: vec3::Vec3::new(arg.1.0, arg.1.1, arg.1.2),
            horizontal: vec3::Vec3::new(arg.2.0, arg.2.1, arg.2.2),
            vertical: vec3::Vec3::new(arg.3.0, arg.3.1, arg.3.2),
            u: vec3::Vec3::new(arg.4.0, arg.4.1, arg.4.2),
            v: vec3::Vec3::new(arg.5.0, arg.5.1, arg.5.2),
            w: vec3::Vec3::new(arg.6.0, arg.6.1, arg.6.2),
            lens_radius: arg.7
        }
    }
}

/*
 * Returns random Vec3 Vec3(0.0-1.0, 0.0-1.0, 0.0)
 */
fn random_in_unit_disk(seed: &mut u32) -> vec3::Vec3 {
    let mut p = 2.0 * vec3::Vec3::new(util::randf(seed), util::randf(seed), 0.0) - vec3::Vec3::new(1.0, 1.0, 0.0);

    while p.dot(&p) >= 1.0 {
        p = 2.0 * vec3::Vec3::new(util::randf(seed), util::randf(seed), 0.0) - vec3::Vec3::new(1.0, 1.0, 0.0);
    }
    p
}
