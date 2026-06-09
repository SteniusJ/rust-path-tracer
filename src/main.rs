use cuda_core::CudaContext;

mod path_tracer;

use path_tracer::{
    camera::Camera,
    vec3::Vec3,
    materials::Material,
    geometry
};

fn main() {
    let px_width = 400;
    let px_height = 200;
    let samples = 10;
    let seed = 12345;
    let depth = 50;
    let denoising = 3; // width of window used for median filter denoising, x < 2 to disable

    let look_from = Vec3::new(3.0, 2.0, 0.5);
    let look_at = Vec3::new(-2.0, 2.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 60.0;
    let aperature = 0.02;
    let dist_to_focus = (look_from - look_at).len();
    let aspect = px_width as f64 / px_height as f64;
    
    let camera = Camera::new(look_from, look_at, v_up, fov, aspect, aperature, dist_to_focus);

    let cuboid_mat = Material::new_lambertian(Vec3::new(0.5, 0.2, 0.2));
    let _glass = Material::new_dielectric(1.5);
    let metallic = Material::new_metal(Vec3::new(0.5, 0.5, 0.5), 0.0);
    let tri_mat = Material::new_lambertian(Vec3::new(1.0, 0.0, 0.0));

    let mut world: Vec<geometry::Triangle> = Vec::new();

    let _cuboid = geometry::Cuboid::new_to_world(
        Vec3::new(-2.0, 1.0, -2.0),
        Vec3::new(-2.0, 1.0, -4.0), 
        Vec3::new(-2.0, 3.0, -2.0),
        Vec3::new(-2.0, 3.0, -4.0),
        Vec3::new(-4.0, 1.0, -2.0),
        Vec3::new(-4.0, 1.0, -4.0),
        Vec3::new(-4.0, 3.0, -2.0),
        Vec3::new(-4.0, 3.0, -4.0),
        cuboid_mat,
        &mut world
        );

    let _cuboid_metallic = geometry::Cuboid::new_to_world(
        Vec3::new(200.0, -3.0, 200.0),
        Vec3::new(200.0, -3.0, -200.0),
        Vec3::new(200.0, 0.0, 200.0),
        Vec3::new(200.0, 0.0, -200.0),
        Vec3::new(-200.0, -3.0, 200.0),
        Vec3::new(-200.0, -3.0, -200.0),
        Vec3::new(-200.0, 0.0, 200.0),
        Vec3::new(-200.0, 0.0, -200.0),
        metallic,
        &mut world
        );
/*
    let _custom_obj = geometry::ObjImport::new_to_world(
        "suzanne.obj",
        glass,
        &mut world
        );
*/

    world.push(geometry::Triangle::new(
                Vec3::new(-2.0, 1.0, 4.0),
                Vec3::new(-2.0, 1.0, 2.0),
                Vec3::new(-2.0, 3.0, 3.0),
                tri_mat
                ));

    let ctx = CudaContext::new(0).expect("Failed to create CUDA context");
    let stream = ctx.default_stream();
    let module = path_tracer::kernels::load(&ctx).expect("Failed to load module");

    path_tracer::render(
        px_width,
        px_height,
        samples,
        depth,
        seed,
        world,
        camera,
        "output.ppm",
        denoising,
        module,
        stream
        );
}
