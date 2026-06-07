pub mod camera;
pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod util;
pub mod geometry;
pub mod materials;
pub mod output;

use cuda_device::{kernel, thread, DisjointSlice, gpu_printf};
use cuda_core::{DeviceBuffer, LaunchConfig, CudaStream};
use cuda_host::cuda_module;

use std::fs::File;
use std::io::Write;
use std::sync::Arc;

#[cuda_module]
pub mod kernels {
    use super::{
        camera,
        vec3,
        util,
        geometry,
        kernel,
        thread,
        DisjointSlice,
        gpu_printf,
        get_color
    };

    #[kernel]
    pub fn render(
        tris: &[geometry::Triangle],
        camera: (
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            (f64, f64, f64),
            f64
            ),
        samples: u8,
        px_width: u16,
        px_height: u16,
        depth: u8,
        mut out: DisjointSlice<((f64, f64, f64), (f64, f64, f64))>
        ) {
        let idx = thread::index_1d();
        let i = idx.get();

        if let Some(out_elem) = out.get_mut(idx) {
            let mut color = vec3::Vec3::empty();
            let mut seed = i as u32 + 23712;
            let camera = camera::Camera::from_gpu_arg(camera);

            let j = px_height as usize - (i / px_width as usize);
            let i = i - (i / px_width as usize * px_width as usize);

            for _ in 0..samples {
                let u = (i as f64 + util::randf(&mut seed)) / px_width as f64;
                let v = (j as f64 + util::randf(&mut seed)) / px_height as f64;
                let ray = camera.get_ray(u, v, &mut seed);
                let org = ray.origin;
                let dir = ray.direction;
                *out_elem = ((org.x, org.y, org.z), (dir.x, dir.y, dir.z));
                //color += get_color(ray, tris, depth, &mut seed);
            }
/*
            color /= samples as f64;
            color = vec3::Vec3::new(
                util::sqrt_f64(color.x),
                util::sqrt_f64(color.y),
                util::sqrt_f64(color.z)
                );
            let color = color.to_color();
            *out_elem = (color.r, color.g, color.b);
*/
        }
    }
}

pub fn render(
    px_width: u16,
    px_height: u16,
    samples: u8,
    depth: u8,
    world: Vec<geometry::Triangle>,
    camera: camera::Camera,
    output_name: &str,
    denoising: u8,
    module: kernels::LoadedModule,
    stream: Arc<CudaStream>
    ) {
    let mut output = File::create("gpu_rays.txt").unwrap();
    // due to denoising removing the edges, we make the initial render bigger by the window
    // width(denoising)
    let px_width = px_width + denoising as u16;
    let px_height = px_height + denoising as u16;
    let mut render_data = output::RenderPPM::new(px_width, px_height, 255);
    let npixels = px_width as u32 * px_height as u32;

    let tris_dev = DeviceBuffer::from_host(&stream, &world).unwrap();

    let mut out_dev = DeviceBuffer::<((f64, f64, f64), (f64, f64, f64))>::zeroed(&stream, npixels as usize).unwrap();

    module.
        render(
            &stream,
            LaunchConfig::for_num_elems(npixels),
            &tris_dev,
            camera.into_gpu_arg(),
            samples,
            px_width,
            px_height,
            depth,
            &mut out_dev
            )
        .expect("Kernel launch failed");

    let out = out_dev.to_host_vec(&stream).unwrap();

    output.write_all(format!("{out:?}").as_bytes()).unwrap();

/*
    render_data.push_gpu_vec(out);

    if denoising > 1 {
        println!("\nstarting denoising...");
        render_data.median_filter(denoising, 1);
    }

    output.write_all(render_data.to_string().as_bytes()).unwrap();
*/
}

fn check_hits(ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hitable::HitRecord, tris: &[geometry::Triangle]) -> bool {
    let mut temp_rec: hitable::HitRecord = hitable::HitRecord::empty();
    let mut hit = false;
    let mut closest_t = t_max;

    for tri in tris {
        if tri.hit(ray, t_min, closest_t, &mut temp_rec) {
            hit = true;
            if closest_t > temp_rec.t {
                closest_t = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
    }

    hit
}

fn get_color(ray: ray::Ray, tris: &[geometry::Triangle], max_depth: u8, seed: &mut u32) -> vec3::Vec3 {
    let mut depth = 0;
    let mut attentuation = vec3::Vec3::new(1.0, 1.0, 1.0);
    let mut ray = ray;

    loop {
        let mut hit_record = hitable::HitRecord::empty();
        let mut loop_attentuation = vec3::Vec3::empty();
        let mut scattered = ray::Ray::empty();

        // max depth reached, loop ends
        if depth >= max_depth {
            return attentuation * vec3::Vec3::empty();
        }

        // Ray didn't hit anything, loop ends
        if !check_hits(&ray, 0.001, f64::MAX, &mut hit_record, tris) {
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
