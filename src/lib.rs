pub mod vec3;
pub mod ray;
pub mod hitable;
pub mod geometry;
pub mod materials;
pub mod camera;
mod util;
mod output;

use cuda_device::{kernel, thread, DisjointSlice};
use cuda_host::cuda_module;
use cuda_core::{CudaContext, DeviceBuffer, LaunchConfig, DeviceCopy};

use std::fs::File;
use std::io::prelude::*;
use std::io;
use rand::rngs::SmallRng;

#[cuda_module]
mod kernels {
    use super::*;

    #[kernel]
    fn render(tris: &[geometry::Triangle], randnr: &[f64], camera: camera::Camera, samples: u8, px_width: u16, px_height: u16, mut out: DisjointSlice<(u8, u8, u8)>) {
        let idx = thread::index_1d();
        let i = idx.get();
        if let Some(out_elem) = out.get_mut(idx) {
            let randnr = randnr[i];
            let mut color = vec3::Vec3::empty();

            let ix = i / px_width as usize;
            let iy = i / px_height as usize;

            for _ in 0..samples {
                let u = (ix as f64 + randnr) / px_width as f64;
                let v = (iy as f64 + randnr) / px_height as f64;

                let ray = camera.get_ray(u, v, randnr);

                color += get_color(&ray, tris, 50, materials::Material::new_lambertian(vec3::Vec3::empty()), randnr);
            }

            color /= samples as f64;
            color = vec3::Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
            let color = color.to_color();

            *out_elem = (color.r, color.g, color.b);
        }
    }
}

fn get_color(ray: &ray::Ray, tris: &[geometry::Triangle], depth: u8, default_mat: materials::Material, randnr: f64) -> vec3::Vec3 {
    vec3::Vec3::empty()
}

pub fn render(px_width: u16, px_height: u16, samples: u8, world: Vec<geometry::Triangle>, camera: camera::Camera, output_name: &str, default_mat: materials::Material, prog_interval: i64, denoising: u8) {
    let mut progress = 0.0;
    let mut output = File::create(output_name).unwrap();
    let mut rng: SmallRng = rand::make_rng();
    // due to denoising removing the edges, we make the initial render bigger by the window
    // width(denoising)
    let px_width = px_width + denoising as u16;
    let px_height = px_height + denoising as u16;
    let mut render_data = output::RenderPPM::new(px_width, px_height, 255);
    let pixels = px_width as usize * px_height as usize;

    // CUDA rewrite
    let ctx = CudaContext::new(0).expect("Failed to create CUDA context");
    let stream = ctx.default_stream();

    let randnr_vec = util::get_randnr_vec(world.len(), &mut rng);

    let tris_dev = DeviceBuffer::from_host(&stream, &world).unwrap();
    let randnr_dev = DeviceBuffer::from_host(&stream, &randnr_vec).unwrap();

    let out_dev = DeviceBuffer::<(u8, u8, u8)>::zeroed(&stream, pixels).unwrap();

    /* IN THEORY:
     *
     * module
     *      .render(
     *          &stream,
     *          LaunchConfig::for_num_elems(pixels),
     *          &tris_dev,
     *          &randnr_dev,
     *          &mut out_dev,
     *          camera,
     *          samples
     *          )
     *      .expect("Kernel launch failed");
     *
     *  let out_host = DeviceBuffer::to_host_vec(&out_dev, &stream);
     *  render_data.push_vec(out_host);
     *
     *  yada yada
     */

    // need to generate a vector of random values since this cannot be done on the GPU
    // vector of triangles into device vector
    // empty vector on device of image dimensions size for color data

    // CUDA end

}
