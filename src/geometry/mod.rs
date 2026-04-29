use crate::{vec3, materials, hittable, ray};
use std::fs::File;
use std::io::Read;

pub struct Triangle {
    pub vertice1: vec3::Vec3,
    pub vertice2: vec3::Vec3,
    pub vertice3: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub material: &'static dyn materials::Material,
}

impl Triangle {
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
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, material: &'static dyn materials::Material) -> Triangle {
        Triangle {
            vertice1: v1,
            vertice2: v2,
            vertice3: v3,
            normal: (v2 - v1).cross(&(v3 - v1)).to_normalized(),
            material
        }
    }
}

impl hittable::Hittable for Triangle {
    fn hit(&self, ray: &ray::Ray, _t_min: f64, _t_max: f64, rec: &mut hittable::HitRecord) -> bool {
        let r_dir = ray.direction.to_normalized();

        if self.normal.dot(&r_dir) == 0.0 {
            return false;
        }

        let t = self.normal.dot(&(self.vertice1 - ray.origin)) / self.normal.dot(&r_dir);

        // Triangle behind camera
        if t <= 0.0 {
            return false;
        }

        let int_point = ray.origin + r_dir * t;

        let edge_1_2 = self.vertice1 - self.vertice2;
        let edge_1_3 = self.vertice1 - self.vertice3;
        let edge_int = self.vertice1 - int_point;

        let denominator = (edge_1_2.dot(&edge_1_2) * edge_1_3.dot(&edge_1_3)) - (edge_1_2.dot(&edge_1_3) * edge_1_2.dot(&edge_1_3));
        let u = ((edge_1_2.dot(&edge_int) * edge_1_3.dot(&edge_1_3)) - (edge_1_2.dot(&edge_1_3) * edge_1_3.dot(&edge_int))) / denominator;
        let v = ((edge_1_2.dot(&edge_1_2) * edge_1_3.dot(&edge_int)) - (edge_1_2.dot(&edge_1_3) * edge_1_2.dot(&edge_int))) / denominator;

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

pub struct Cuboid {
    pub triangles: *mut Triangle,
}

impl Cuboid {
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
    pub fn new(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, v4: vec3::Vec3, v5: vec3::Vec3, v6: vec3::Vec3, v7: vec3::Vec3, v8: vec3::Vec3, material: &'static dyn materials::Material) -> (Cuboid, Vec<Triangle>) {
        let mut triangles: Vec<Triangle> = Vec::with_capacity(12);
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
    pub fn new_to_world(v1: vec3::Vec3, v2: vec3::Vec3, v3: vec3::Vec3, v4: vec3::Vec3, v5: vec3::Vec3, v6: vec3::Vec3, v7: vec3::Vec3, v8: vec3::Vec3, material: &'static dyn materials::Material, world: &mut Vec<Box<dyn hittable::Hittable>>) -> Cuboid {
        let cuboid = Cuboid::new(v1, v2, v3, v4, v5, v6, v7, v8, material);

        world.reserve(12);
        for tri in cuboid.1 {
            world.push(Box::new(tri));
        }

        cuboid.0
    }
}

pub struct ObjImport {
    pub triangles: *mut Triangle,
}

impl ObjImport {
    /* Constructs new Custom model from .obj wavefront file.
     * Doesn't auto triangulate, requires mesh to be pre triangulated.
     */
    pub fn new(file_name: &str, material: &'static dyn materials::Material) -> (ObjImport, Vec<Triangle>) {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut import_file = File::open(file_name).unwrap();
        let mut file_contents = String::new();
        let mut vertices: Vec<vec3::Vec3> = Vec::new();

        import_file.read_to_string(&mut file_contents).unwrap();

        for line in file_contents.lines() {
            let vertex_data: Vec<&str> = line.split(' ').collect();

            match vertex_data[0] {
                "v" => {
                    let x = vertex_data[1].parse::<f64>().unwrap();
                    let y = vertex_data[2].parse::<f64>().unwrap();
                    let z = vertex_data[3].parse::<f64>().unwrap();

                    vertices.push(vec3::Vec3::new(x, y, z));
                },
                "f" => {
                    // If triangles vector has no pushes reserves an aproximated amount of memory
                    if triangles.is_empty() {
                        triangles.reserve(vertices.len() / 3);
                    }

                    /* 
                     * OBJ files store face data as "f 1/2/1 3/5/3 7/6/5"
                     * We only care for the first number hence the ugly one-liner where we seperate
                     * only the first index
                     *
                     * Indexes are also stored starting from 1 which means we have to subtract by 1
                     * for our 0 indexing.
                     */
                    let vert1_idx = vertex_data[1].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                    let vert2_idx = vertex_data[2].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                    let vert3_idx = vertex_data[3].split('/').next().unwrap().parse::<usize>().unwrap() - 1;

                    triangles.push(Triangle::new(vertices[vert1_idx], vertices[vert2_idx], vertices[vert3_idx], material));
                },
                _ => (),
            }
        }

        (
            ObjImport {
                triangles: triangles.as_mut_ptr(),
            },
            triangles
        )
    }
    pub fn new_to_world(file_name: &str, material: &'static dyn materials::Material, world: &mut Vec<Box<dyn hittable::Hittable>>) -> ObjImport {
        let import = ObjImport::new(file_name, material);

        world.reserve(import.1.len());
        for tri in import.1 {
            world.push(Box::new(tri));
        }

        import.0
    }
}
