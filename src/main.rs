use path_tracer;

fn main() {
    let px_width = 1280;
    let px_height = 720;
    let samples = 5;

    let look_from = path_tracer::vec3::Vec3::new(3.0, 2.0, 0.5);
    let look_at = path_tracer::vec3::Vec3::new(-2.0, 2.0, 0.0);
    let v_up = path_tracer::vec3::Vec3::new(0.0, 1.0, 0.0);
    let fov = 60.0;
    let aperature = 0.02;
    let dist_to_focus = (look_from - look_at).len();
    let aspect = px_width as f64 / px_height as f64;
    let camera = path_tracer::camera::Camera::new(look_from, look_at, v_up, fov, aspect, aperature, dist_to_focus);

    let default_mat = Box::new(path_tracer::materials::Lambertian::new(path_tracer::vec3::Vec3::empty()));
    let cuboid_mat = Box::new(path_tracer::materials::Lambertian::new(path_tracer::vec3::Vec3::new(0.5, 0.2, 0.2)));
    let tri_mat = Box::new(path_tracer::materials::Lambertian::new(path_tracer::vec3::Vec3::new(1.0, 0.0, 0.0)));

    let mut world: Vec<Box<dyn path_tracer::hittable::Hittable>> = Vec::new();

    let cuboid = path_tracer::geometry::Cuboid::new(
        path_tracer::vec3::Vec3::new(-2.0, 1.0, -2.0),
        path_tracer::vec3::Vec3::new(-2.0, 1.0, -4.0), 
        path_tracer::vec3::Vec3::new(-2.0, 3.0, -2.0),
        path_tracer::vec3::Vec3::new(-2.0, 3.0, -4.0),
        path_tracer::vec3::Vec3::new(-4.0, 1.0, -2.0),
        path_tracer::vec3::Vec3::new(-4.0, 1.0, -4.0),
        path_tracer::vec3::Vec3::new(-4.0, 3.0, -2.0),
        path_tracer::vec3::Vec3::new(-4.0, 3.0, -4.0),
        Box::leak(cuboid_mat.clone())
        );

    for tri in cuboid.1 {
        world.push(Box::new(tri));
    }

    let custom_obj = path_tracer::geometry::ObjImport::new(
        "suzanne.obj",
        Box::leak(cuboid_mat)
        );

    for tri in custom_obj.1 {
        world.push(Box::new(tri));
    }

    world.push(Box::new(path_tracer::geometry::Triangle::new(
                path_tracer::vec3::Vec3::new(-2.0, 1.0, 4.0),
                path_tracer::vec3::Vec3::new(-2.0, 1.0, 2.0),
                path_tracer::vec3::Vec3::new(-2.0, 3.0, 3.0),
                Box::leak(tri_mat)
                )));

    path_tracer::render(
        px_width,
        px_height,
        samples,
        world,
        camera,
        "output.ppm",
        Box::leak(default_mat),
        1
        );
}
