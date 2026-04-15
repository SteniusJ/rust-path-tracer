use crate::{ray, vec3, hittable, materials};

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
