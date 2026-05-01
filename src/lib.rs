pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod geometry;
pub mod materials;
pub mod camera;
mod util;
mod output;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use rand::rngs::SmallRng;

fn check_hits(ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hitable::HitRecord, world: &Vec<Box<dyn hitable::Hitable>>, default_mat: &'static dyn materials::Material) -> bool {
    let mut temp_rec: hitable::HitRecord = hitable::HitRecord::empty(default_mat);
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

fn get_color(ray: &ray::Ray, world: &Vec<Box<dyn hitable::Hitable>>, depth: u8, default_mat: &'static dyn materials::Material, rng: &mut SmallRng) -> vec3::Vec3 {
    let mut hit_record: hitable::HitRecord = hitable::HitRecord::empty(default_mat);

    if check_hits(ray, 0.001, f64::MAX, &mut hit_record, world, default_mat) {
        let mut scattered = ray::Ray::empty();
        let mut attentuation = vec3::Vec3::empty();

        if depth < 50 && hit_record.material.scatter(ray, &hit_record, &mut attentuation, &mut scattered, rng) {
            return attentuation * get_color(&scattered, world, depth + 1, default_mat, rng);
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

pub fn render(px_width: u16, px_height: u16, samples: u8, world: Vec<Box<dyn hitable::Hitable>>, camera: camera::Camera, output_name: &str, default_mat: &'static dyn materials::Material, prog_interval: i64) {
    let mut progress = 0.0;
    let mut output = File::create(output_name).unwrap();
    let mut rng: SmallRng = rand::make_rng();
    let mut render_data = output::RenderPPM::new(px_width, px_height, 255);

    let mut j = px_height.clone();
    while j >= 1 {
        let mut i = 0;
        while i < px_width {
            let mut color = vec3::Vec3::empty();

            for _ in 0..samples {
                let u = (i as f64 + util::randf(&mut rng)) / px_width as f64;
                let v = (j as f64 + util::randf(&mut rng)) / px_height as f64;
                let ray = camera.get_ray(u, v, &mut rng);
                color += get_color(&ray, &world, 0, default_mat, &mut rng);
            }

            color /= samples as f64;
            // gamma correction
            color = vec3::Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            render_data.push(color.to_color());

            i += 1;
        }

        progress += 100.0 / px_height as f64;
        if progress as i64 % prog_interval == 0 {
            print!("\rrender progress: {progress}%");
            io::stdout().flush().unwrap();
        }

        j -= 1;
    }

    output.write_all(render_data.to_string().as_bytes()).unwrap();
    output.flush().unwrap();
}
