pub mod camera;
pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod util;
pub mod geometry;
pub mod materials;

use cuda_device::{kernel, thread, DisjointSlice, gpu_printf};
use cuda_core::{DeviceBuffer, LaunchConfig, CudaStream};
use cuda_host::cuda_module;

use std::fs::File;
use std::sync::Arc;

#[cuda_module]
pub mod kernels {
    use super::{
        camera,
        vec3,
        util,
        geometry,
        materials,
        kernel,
        thread,
        DisjointSlice,
        gpu_printf,
        get_color
    };

    #[kernel]
    pub fn render(tris: &[geometry::Triangle], camera: camera::Camera, samples: u8, px_width: u16, px_height: u16, mut out: DisjointSlice<(u8, u8, u8)>) {
        let idx = thread::index_1d();
        let i = idx.get();
        let mut seed = i as u32 + 23712;

        if let Some(out_elem) = out.get_mut(idx) {
            let mut color = vec3::Vec3::empty();

            let ix = i / px_width as usize;
            let iy = i / px_height as usize;

            for _ in 0..samples {
                let u = (ix as f64 + util::randf(&mut seed)) / px_width as f64;
                let v = (iy as f64 + util::randf(&mut seed)) / px_height as f64;

                let ray = camera.get_ray(u, v, &mut seed);

                color += get_color(&ray, tris, 50, materials::Material::new_lambertian(vec3::Vec3::empty()), &mut seed);
            }

            color /= samples as f64;
            color = vec3::Vec3::new(
                util::sqrt_f64(color.x),
                util::sqrt_f64(color.y),
                util::sqrt_f64(color.z)
                );
            let color = color.to_color();

            *out_elem = (color.r, color.g, color.b);
        }
    }
}

pub fn render(
    px_width: u16,
    px_height: u16,
    samples: u8,
    world: Vec<geometry::Triangle>,
    camera: camera::Camera,
    output_name: &str,
    _default_mat: materials::Material,
    _prog_interval: i64,
    denoising: u8,
    module: kernels::LoadedModule,
    stream: Arc<CudaStream>
    ) {
    let mut _progress = 0.0;
    let mut _output = File::create(output_name).unwrap();
    // due to denoising removing the edges, we make the initial render bigger by the window
    // width(denoising)
    let px_width = px_width + denoising as u16;
    let px_height = px_height + denoising as u16;
    //let mut _render_data = output::RenderPPM::new(px_width, px_height, 255);
    let npixels = px_width as u32 * px_height as u32;

    let tris_dev = DeviceBuffer::from_host(&stream, &world).unwrap();

    let mut out_dev = DeviceBuffer::<(u8, u8, u8)>::zeroed(&stream, npixels as usize).unwrap();

    module.
        render(
            &stream,
            LaunchConfig::for_num_elems(npixels),
            &tris_dev,
            camera,
            samples,
            px_width,
            px_height,
            &mut out_dev
            )
        .expect("Kernel launch failed");

    let out = out_dev.to_host_vec(&stream).unwrap();

    println!("{}, {}, {}", out[20].0, out[20].1, out[20].2);
}

fn check_hits(ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hitable::HitRecord, tris: &[geometry::Triangle], default_mat: materials::Material) -> bool {
    let mut temp_rec: hitable::HitRecord = hitable::HitRecord::empty(default_mat);
    let mut hit = false;
    let mut closest_t = t_max;

    for tri in tris {
        if tri.hit(ray, t_min, closest_t, &mut temp_rec) {
            hit = true;
            if closest_t > temp_rec.t {
                closest_t = temp_rec.t;
                *rec =  temp_rec.clone();
            }
        }
    }

    hit
}

fn get_color(ray: &ray::Ray, tris: &[geometry::Triangle], depth: u8, default_mat: materials::Material, seed: &mut u32) -> vec3::Vec3 {
    let mut hit_record: hitable::HitRecord = hitable::HitRecord::empty(default_mat);

    if check_hits(ray, 0.001, f64::MAX, &mut hit_record, tris, default_mat) {
        let mut scattered = ray::Ray::empty();
        let mut attentuation = vec3::Vec3::empty();

        if depth < 50 && materials::scatter(hit_record.material, ray, &hit_record, &mut attentuation, &mut scattered, seed) {
            return attentuation * get_color(&scattered, tris, depth + 1, default_mat, seed);
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
