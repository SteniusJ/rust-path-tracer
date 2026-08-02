pub mod camera;
pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod util;
pub mod geometry;
pub mod materials;
pub mod output;
pub mod bvh;

use cuda_device::{kernel, thread, DisjointSlice};
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
        bvh,
        kernel,
        thread,
        DisjointSlice,
        get_color
    };

    #[kernel]
    pub fn render(
        tris: &[geometry::Triangle],
        bvh_nodes: &[bvh::BvhNode],
        tri_indices: &[usize],
        camera: camera::CameraFlattened,
        samples: u8,
        px_width: u16,
        px_height: u16,
        depth: u8,
        seed: u32,
        mut out: DisjointSlice<(u8, u8, u8)>
        ) {
        let idx = thread::index_1d();
        let i = idx.get();

        if let Some(out_elem) = out.get_mut(idx) {
            let mut color = vec3::Vec3::empty();
            let mut seed = i as u32 + seed;
            let camera = camera::Camera::from_gpu_arg(camera);
            let bvh = bvh::Bvh::new(bvh_nodes, tri_indices, tris);

            let j = px_height as usize - (i / px_width as usize);
            let i = i - (i / px_width as usize * px_width as usize);

            for _ in 0..samples {
                let u = (i as f64 + util::randf(&mut seed)) / px_width as f64;
                let v = (j as f64 + util::randf(&mut seed)) / px_height as f64;
                let ray = camera.get_ray(u, v, &mut seed);
                color += get_color(ray, /*tris*/&bvh, depth, &mut seed);
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
    depth: u8,
    seed: u32,
    world: Vec<geometry::Triangle>,
    camera: camera::Camera,
    output_name: &str,
    denoising: u8,
    module: kernels::LoadedModule,
    stream: Arc<CudaStream>
    ) {
    let mut output = File::create(output_name).unwrap();
    // due to denoising removing the edges, we make the initial render bigger by the window
    // width(denoising)
    let px_width = px_width + (denoising as u16).div_ceil(2);
    let px_height = px_height + (denoising as u16).div_ceil(2);
    let mut render_data = output::RenderPPM::new(px_width, px_height, 255);
    let npixels = px_width as u32 * px_height as u32;

    let (
        bvh_nodes_dev,
        tri_indices_dev,
        tris_dev
    ) = bvh::build_bvh(&stream, &world);

    let mut out_dev = DeviceBuffer::<(u8, u8, u8)>::zeroed(&stream, npixels as usize).unwrap();

    {
        let denoising = (denoising as u16).div_ceil(2);
        let px_width = px_width - denoising;
        let px_height = px_height - denoising;
        let n_tris = world.len();

        println!("starting render on gpu...
width: {px_width} + {denoising}
height: {px_height} + {denoising}
total pixels: {npixels}
triangles: {n_tris}
samples: {samples}\n",
        );
    }

    unsafe {
        module.
            render(
                &stream,
                LaunchConfig::for_num_elems(npixels),
                &tris_dev,
                &bvh_nodes_dev,
                &tri_indices_dev,
                camera.into_gpu_arg(),
                samples,
                px_width,
                px_height,
                depth,
                seed,
                &mut out_dev
                )
            .expect("Kernel launch failed");
    }

    let out = out_dev.to_host_vec(&stream).unwrap();

    render_data.push_gpu_vec(out);

    if denoising > 2 {
        println!("starting denoising...");
        render_data.median_filter(denoising, 1);
    }

    output.write_all(render_data.to_string().as_bytes()).unwrap();
}

fn get_color(ray: ray::Ray, bvh: &bvh::Bvh, max_depth: u8, seed: &mut u32) -> vec3::Vec3 {
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
        if !bvh.intersect(&ray, &mut hit_record) {
            let unit_direction = ray.direction.normalized();
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
