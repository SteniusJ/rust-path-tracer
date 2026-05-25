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
