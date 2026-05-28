use path_tracer::{vec3, camera, materials, geometry};

fn main() {
    let px_width = 400;
    let px_height = 200;
    let samples = 10;

    let look_from = vec3::Vec3::new(3.0, 2.0, 0.5);
    let look_at = vec3::Vec3::new(-2.0, 2.0, 0.0);
    let v_up = vec3::Vec3::new(0.0, 1.0, 0.0);
    let fov = 60.0;
    let aperature = 0.02;
    let dist_to_focus = (look_from - look_at).len();
    let aspect = px_width as f64 / px_height as f64;
    let camera = camera::Camera::new(look_from, look_at, v_up, fov, aspect, aperature, dist_to_focus);

    let default_mat = materials::Material::new_lambertian(vec3::Vec3::empty());
    let cuboid_mat = materials::Material::new_lambertian(vec3::Vec3::new(0.5, 0.2, 0.2));
    let _glass = materials::Material::new_dielectric(1.5);
    let metallic = materials::Material::new_metal(vec3::Vec3::new(0.5, 0.5, 0.5), 0.0);
    let tri_mat = materials::Material::new_lambertian(vec3::Vec3::new(1.0, 0.0, 0.0));

    let mut world: Vec<geometry::Triangle> = Vec::new();

    let _cuboid = geometry::Cuboid::new_to_world(
        vec3::Vec3::new(-2.0, 1.0, -2.0),
        vec3::Vec3::new(-2.0, 1.0, -4.0), 
        vec3::Vec3::new(-2.0, 3.0, -2.0),
        vec3::Vec3::new(-2.0, 3.0, -4.0),
        vec3::Vec3::new(-4.0, 1.0, -2.0),
        vec3::Vec3::new(-4.0, 1.0, -4.0),
        vec3::Vec3::new(-4.0, 3.0, -2.0),
        vec3::Vec3::new(-4.0, 3.0, -4.0),
        cuboid_mat,
        &mut world
        );

    let _cuboid_metallic = geometry::Cuboid::new_to_world(
        vec3::Vec3::new(200.0, -3.0, 200.0),
        vec3::Vec3::new(200.0, -3.0, -200.0),
        vec3::Vec3::new(200.0, 0.0, 200.0),
        vec3::Vec3::new(200.0, 0.0, -200.0),
        vec3::Vec3::new(-200.0, -3.0, 200.0),
        vec3::Vec3::new(-200.0, -3.0, -200.0),
        vec3::Vec3::new(-200.0, 0.0, 200.0),
        vec3::Vec3::new(-200.0, 0.0, -200.0),
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
                vec3::Vec3::new(-2.0, 1.0, 4.0),
                vec3::Vec3::new(-2.0, 1.0, 2.0),
                vec3::Vec3::new(-2.0, 3.0, 3.0),
                tri_mat
                ));

    path_tracer::render(
        px_width,
        px_height,
        samples,
        world,
        camera,
        "output.ppm",
        default_mat,
        1,
        3
        );
}
