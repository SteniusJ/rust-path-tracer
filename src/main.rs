use path_tracer;

fn main() {
    let px_width = 1280;
    let px_height = 720;
    let samples = 5;

    let look_from = path_tracer::vec3::Vec3::new(1.0, 1.0, 1.0);
    let look_at = path_tracer::vec3::Vec3::new(0.0, 0.0, 0.0);
    let v_up = path_tracer::vec3::Vec3::new(0.0, 1.0, 0.0);
    let fov = 65.0;
    let aperature = 0.2;
    let dist_to_focus = (look_from - look_at).len();
    let camera = path_tracer::camera::Camera::new(look_from, look_at, v_up, fov, (px_width / px_height) as f64, aperature, dist_to_focus);

    let default_mat = path_tracer::materials::Lambertian::new(path_tracer::vec3::Vec3::empty());
    let cuboid_mat = path_tracer::materials::Lambertian::new(path_tracer::vec3::Vec3::new(0.5, 0.5, 1.0));

    let mut world: Vec<Box<dyn path_tracer::hittable::Hittable>> = Vec::new();

    let cuboid = path_tracer::geometry::Cuboid::new(
        path_tracer::vec3::Vec3::new(-0.5, -0.5, 0.5),
        path_tracer::vec3::Vec3::new(0.5, -0.5, 0.5), 
        path_tracer::vec3::Vec3::new(-0.5, 0.5, 0.5),
        path_tracer::vec3::Vec3::new(0.5, 0.5, 0.5),
        path_tracer::vec3::Vec3::new(-0.5, -0.5, -0.5),
        path_tracer::vec3::Vec3::new(0.5, -0.5, -0.5),
        path_tracer::vec3::Vec3::new(-0.5, 0.5, -0.5),
        path_tracer::vec3::Vec3::new(0.5, 0.5, -0.5),
        &cuboid_mat
        );

    for tri in cuboid.1 {
        world.push(Box::new(tri));
    }

    path_tracer::render(
        px_width,
        px_height,
        samples,
        world,
        camera,
        "output.ppm",
        &default_mat,
        5
        );
}
