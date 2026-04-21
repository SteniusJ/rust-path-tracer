use crate::{vec3, materials, hittable, ray};

pub struct Triangle<'a> {
    pub vertice1: vec3::Vec3,
    pub vertice2: vec3::Vec3,
    pub vertice3: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub material: &'a dyn materials::Material,
}

impl<'a> Triangle<'a> {
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
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, material: &'a dyn materials::Material) -> Triangle<'a> {
        Triangle {
            vertice1: v1,
            vertice2: v2,
            vertice3: v3,
            normal: (v2 - v1).cross(&(v3 - v1)).to_normalized(),
            material
        }
    }
}

impl<'a> hittable::Hittable<'a> for Triangle<'a> {
    fn hit(&self, ray: &ray::Ray, _t_min: f64, _t_max: f64, rec: &mut hittable::HitRecord<'a>) -> bool {
        let r_dir = ray.direction.to_normalized();

        if self.normal.dot(&r_dir) == 0.0 {
            return false;
        }

        let t = self.normal.dot(&(self.vertice1 - ray.origin)) / self.normal.dot(&r_dir);

        // Triangle behind camera
        if t <= 0.0 {
            return false;
        }

        let int_point = ray.origin + ray.direction * t;

        let edge_1_2 = self.vertice1 - self.vertice2;
        let edge_1_3 = self.vertice1 - self.vertice3;
        let edge_int = self.vertice1 - int_point;

        let denominator = (edge_1_2.dot(&edge_1_2) * edge_1_3.dot(&edge_1_3)) - (edge_1_2.dot(&edge_1_3) * edge_1_2.dot(&edge_1_3));
        let u = ((edge_1_2.dot(&edge_int) * edge_1_3.dot(&edge_1_3)) - (edge_1_2.dot(&edge_1_3) * edge_1_3.dot(&edge_int))) / denominator;
        let v = ((edge_1_2.dot(&edge_1_2) * edge_1_3.dot(&edge_int)) * (edge_1_2.dot(&edge_1_3) * edge_1_2.dot(&edge_int))) / denominator;

        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            rec.surface_normal = self.normal;
            rec.p = int_point;
            rec.t = (ray.origin - int_point).len();
            rec.material = self.material;

            return true;
        }
        
        false
    }
}

pub struct Cuboid<'a> {
    pub triangles: *mut Triangle<'a>,
}

impl<'a> Cuboid<'a> {
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
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, v4: vec3::Vec3, v5: vec3::Vec3, v6: vec3::Vec3, v7: vec3::Vec3, v8: vec3::Vec3, material: &'a dyn materials::Material) -> (Cuboid<'a>, Vec<Triangle<'a>>) {
        let mut triangles: Vec<Triangle<'a>> = Vec::with_capacity(12);
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
