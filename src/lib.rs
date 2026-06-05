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

fn check_hits(ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hitable::HitRecord, world: &Vec<geometry::Triangle>, default_mat: materials::Material) -> bool {
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

fn get_color(ray: ray::Ray, tris: &Vec<geometry::Triangle>, max_depth: u8, default_mat: materials::Material, seed: &mut u32) -> vec3::Vec3 {
    let mut depth = 0;
    let mut attentuation = vec3::Vec3::new(1.0, 1.0, 1.0);
    let mut ray = ray;

    loop {
        let mut hit_record = hitable::HitRecord::empty(default_mat);
        let mut loop_attentuation = vec3::Vec3::empty();
        let mut scattered = ray::Ray::empty();

        // max depth reached, loop ends
        if depth >= max_depth {
            return attentuation * vec3::Vec3::empty();
        }

        // Ray didn't hit anything, loop ends
        if !check_hits(&ray, 0.001, f64::MAX, &mut hit_record, tris, default_mat) {
            let unit_direction = ray.direction.to_normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            let color = (1.0 - t) * vec3::Vec3::new(1.0, 1.0, 1.0) + t * vec3::Vec3::new(0.5, 0.7, 1.0);
            return attentuation * color;
        }

        // material absorbed ray, loop ends
        if !materials::scatter(&ray, &hit_record, &mut loop_attentuation, &mut scattered, seed) {
            return attentuation * loop_attentuation;
        }

        // ray reflected off surface
        ray = scattered;
        attentuation = attentuation * loop_attentuation;
        depth += 1;
     }
}

pub fn render(px_width: u16, px_height: u16, samples: u8, world: Vec<geometry::Triangle>, camera: camera::Camera, output_name: &str, default_mat: materials::Material, prog_interval: i64, denoising: u8) {
    let mut progress = 0.0;
    let mut output = File::create(output_name).unwrap();
    // due to denoising removing the edges, we make the initial render bigger by the window
    // width(denoising)
    let px_width = px_width + denoising as u16;
    let px_height = px_height + denoising as u16;
    let pixels = px_width as usize * px_height as usize;
    let mut render_data = output::RenderPPM::new(px_width, px_height, 255);

    for px in 0..pixels {
        let j = px_height as usize - (px / px_width as usize);
        let i = px - (px / px_width as usize * px_width as usize);

        let mut seed = px as u32 + 231231;
        let mut color = vec3::Vec3::empty();

        for _ in 0..samples {
            let u = (i as f64 + util::randf(&mut seed)) / px_width as f64;
            let v = (j as f64 + util::randf(&mut seed)) / px_height as f64;
            let ray = camera.get_ray(u, v, &mut seed);
            color += get_color(ray, &world, 50, default_mat, &mut seed);
        }

        color /= samples as f64;
        // gamma correction
        color = vec3::Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
        render_data.push(color.to_color());

        progress += 100.0 / pixels as f64;
        if progress as i64 % prog_interval == 0 {
            print!("\rrender progress: {progress:.2}%");
            io::stdout().flush().unwrap();
        }
    }

    if denoising > 1 {
        println!("\nstarting denoising...");
        render_data.median_filter(denoising, prog_interval);
    }
    println!("\nrender complete!");
    output.write_all(render_data.to_string().as_bytes()).unwrap();
    output.flush().unwrap();
}
