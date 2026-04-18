use crate::{vec3, materials, hittable, ray};

pub struct Triangle<'a, T: materials::Material> {
    pub vertice1: vec3::Vec3,
    pub vertice2: vec3::Vec3,
    pub vertice3: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub material: &'a T,
}

impl<'a, T> Triangle<'a, T> where T: materials::Material {
    /*
     * Constructs a Triangle
     *
     * For a Triangle that has a normal pointing outward (towards the observer (you))
     * the vertices should be placed as follows
     *  
     *        3
     *       / \
     *      /   \
     *     /     \
     *    /       \
     *   /         \
     * 1/___________\2
     *
     */
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, material: &'a T) -> Triangle<'a, T> {
        Triangle {
            vertice1: v1,
            vertice2: v2,
            vertice3: v3,
            normal: (v2 - v1).cross(v3 - v1).to_normalized(),
            material
        }
    }
}

impl<'a, T> hittable::Hittable for Triangle<'a, T> where T: materials::Material {
    fn hit(ray: &ray::Ray, t_min: f32, t_max: f32, rec: &mut hittable::HitRecord) {
        // Implement triangle hit functions here
    }
}

pub struct Cuboid<'a, T: materials::Material> {
    pub triangles: *mut Triangle<'a, T>,
}

impl<'a, T> Cuboid<'a, T> where T: materials::Material {
    /* Constructs new Cuboid
     * Returns vector of Triangle
     *
     * Placement order of vertices. Face 1,2,3,4 is closer to the observer
     *
     *    7________8
     *   /|       /|
     * 3/_______4/ |
     * |  |     |  |
     * |  5_____|__6
     * | /      | /
     * 1/_______2/
     *
     */
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, v4: vec3::Vec3, v5: vec3::Vec3, v6: vec3::Vec3, v7: vec3::Vec3, v8: vec3::Vec3, material: &'a T) -> (Cuboid<'a, T>, Vec<Triangle<'a, T>>) {
        let mut triangles: Vec<Triangle<'a, T>> = Vec::with_capacity(12);
        triangles.push(Triangle::new(v1, v2, v3, material));
        triangles.push(Triangle::new(v3, v2, v4, material));
        triangles.push(Triangle::new(v1, v3, v5, material));
        triangles.push(Triangle::new(v3, v7, v5, material));
        triangles.push(Triangle::new(v1, v2, v5, material));
        triangles.push(Triangle::new(v5, v2, v6, material));
        triangles.push(Triangle::new(v8, v4, v2, material));
        triangles.push(Triangle::new(v2, v6, v8, material));
        triangles.push(Triangle::new(v5, v7, v8, material));
        triangles.push(Triangle::new(v5, v8, v6, material));
        triangles.push(Triangle::new(v3, v8, v7, material));
        triangles.push(Triangle::new(v3, v4, v8, material));

        (
            Cuboid {
                triangles: triangles.as_mut_ptr(),
            },
            triangles
        )
    } 
}
