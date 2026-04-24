pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod geometry;
pub mod materials;
pub mod util;
pub mod camera;

use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::io;

fn check_hits(ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hittable::HitRecord, world: &Vec<Box<dyn hittable::Hittable>>, default_mat: Arc<dyn materials::Material>) -> bool {
    let mut temp_rec: hittable::HitRecord = hittable::HitRecord::empty(default_mat);
    let mut hit = false;
    let mut closest_t = t_max;

    for hittable in world {
        if hittable.hit(ray, t_min, closest_t, &mut temp_rec) {
            hit = true;
            if closest_t > temp_rec.t {
                closest_t = temp_rec.t;
                *rec =  temp_rec.clone();
            }
        }
    }

    hit
}

fn get_color(ray: &ray::Ray, world: &Vec<Box<dyn hittable::Hittable>>, depth: u8, default_mat: Arc<dyn materials::Material>) -> vec3::Vec3 {
    let mut hit_record: hittable::HitRecord = hittable::HitRecord::empty(default_mat.clone());

    if check_hits(ray, 0.001, f64::MAX, &mut hit_record, world, default_mat.clone()) {
        let mut scattered = ray::Ray::empty();
        let mut attentuation = vec3::Vec3::empty();

        if depth < 50 && hit_record.material.scatter(ray, &hit_record, &mut attentuation, &mut scattered) {
            return attentuation * get_color(&scattered, world, depth + 1, default_mat);
        } else {
            return vec3::Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = ray.direction.to_normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color = (1.0 - t) * vec3::Vec3::new(1.0, 1.0, 1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);
        return color;
    }
}

pub fn render(px_width: u16, px_height: u16, samples: u8, world: Vec<Box<dyn hittable::Hittable>>, camera: camera::Camera, output_name: &str, default_mat: Arc<dyn materials::Material>, prog_interval: i64) {
    let mut progress = 0.0;
    let mut output = File::create(output_name).unwrap();

    // write ppm header
    let out = format!("P3\n{px_width} {px_height}\n255\n");
    output.write(out.as_bytes()).unwrap();

    let mut j = px_height.clone();
    while j >= 1 {
        let mut i = 0;
        while i < px_width {
            let mut color = vec3::Vec3::empty();

            for _ in 0..samples {
                let u = i as f64 + util::randf() / px_width as f64;
                let v = i as f64 + util::randf() / px_height as f64;
                let ray = camera.get_ray(u, v);
                color += get_color(&ray, &world, 0, default_mat.clone());
            }

            color /= samples as f64;
            // gamma correction
            color = vec3::Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            let out = format!("{}\n", color.to_color());
            output.write(out.as_bytes()).unwrap();

            i += 1;
        }
        progress += 100.0 / px_height as f64;
        if progress as i64 % prog_interval == 0 {
            print!("\rrender progress: {progress}%");
            io::stdout().flush().unwrap();
        }

        j -= 1;
    }

    output.flush().unwrap();
}
