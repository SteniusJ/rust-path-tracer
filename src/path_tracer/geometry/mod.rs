use crate::path_tracer::{
    vec3::Vec3,
    hitable,
    materials,
    ray
};
use crate::midpoint;
use std::fs::File;
use std::io::Read;

use cuda_core::DeviceCopy;

// "Pushes" object by move_vec
pub fn move_by(world: &mut Vec<Triangle>, obj_ptr: &ObjPointer, move_vec: Vec3) {
    for i in 0..obj_ptr.len {
        let tri = world.get_mut(obj_ptr.ptr + i).unwrap();
        tri.move_by(move_vec);
    }
}

// Moves object relative to its origin to target position (move_vec)
pub fn move_to(world: &mut Vec<Triangle>, obj_ptr: &ObjPointer, move_vec: Vec3) {
    for i in 0..obj_ptr.len {
        let tri = world.get_mut(obj_ptr.ptr + i).unwrap();
        tri.move_to(move_vec);
    }
}

/* Subdivides object "level" times
 *
 * This function only subdivides the mesh and does not smooth it
 *
 * WARNING!
 * Current implementation of the subdivide function includes a major bug related to the function of
 * the object pointers. Since the subdivide function drains the mesh from the world vector it shifts
 * all objects that come after it in the vector causing their pointers to become incorrect.
 * I won't bother fixing this bug since it requires a rewrite of how objects and their pointers are
 * handled.
 */
pub fn subdivide(world: &mut Vec<Triangle>, obj_ptr: &mut ObjPointer, level: u8) {
    let obj_start = world.len() - obj_ptr.len;
    let obj_slice: Vec<Triangle> = world.drain(obj_ptr.ptr..obj_ptr.ptr + obj_ptr.len).collect();

    for tri in obj_slice {
        let result = tri.subdivide(level);
        
        for tri in result {
            world.push(tri);
        }
    }

    obj_ptr.ptr = obj_start;
    obj_ptr.len = world.len() - obj_start;
}

pub struct ObjPointer {
    pub ptr: usize,
    pub len: usize
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertice1: Vec3,
    pub vertice2: Vec3,
    pub vertice3: Vec3,
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: materials::Material,
}

unsafe impl DeviceCopy for Triangle {}

impl Triangle {
    /*
     * Constructs a Triangle
     *
     * For a Triangle that has a normal pointing outward (towards the observer (you))
     * the vertices should be placed as follows
     *  
     *        2
     *       / \
     *      /   \
     *     /     \
     *    /       \
     *   /         \
     * 1/___________\3
     *
     */
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, material: materials::Material) -> Triangle {
        Triangle {
            vertice1: v1,
            vertice2: v2,
            vertice3: v3,
            origin: midpoint!(v1, v2, v3),
            normal: (v2 - v1).cross(&(v3 - v1)).to_normalized(),
            material
        }
    }
    pub fn new_with_origin(v1: Vec3, v2: Vec3, v3: Vec3, material: materials::Material, origin: Vec3) -> Triangle {
        Triangle {
            vertice1: v1,
            vertice2: v2,
            vertice3: v3,
            origin,
            normal: (v2 - v1).cross(&(v3 - v1)).to_normalized(),
            material
        }
    }
    pub fn hit(&self, ray: &ray::Ray, _t_min: f64, _t_max: f64, rec: &mut hitable::HitRecord) -> bool {
        let r_dir = ray.direction.to_normalized();

        if self.normal.dot(&r_dir) > 0.0 {
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

        let denominator = edge_1_2.cross(&edge_1_3).dot(&edge_1_2.cross(&edge_1_3));

        let u = ((edge_1_2.dot(&edge_int) * edge_1_3.dot(&edge_1_3)) - (edge_1_2.dot(&edge_1_3) * edge_1_3.dot(&edge_int))) / denominator;
        let v = ((edge_1_2.dot(&edge_1_2) * edge_1_3.dot(&edge_int)) - (edge_1_2.dot(&edge_1_3) * edge_1_2.dot(&edge_int))) / denominator;

        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            rec.surface_normal = self.normal;
            rec.p = int_point;
            rec.t = (ray.origin - int_point).len();
            rec.uv = u + v;
            rec.material = self.material;

            return true;
        }
        
        false
    }
    pub fn move_by(&mut self, move_vec: Vec3) {
        self.vertice1 += move_vec;
        self.vertice2 += move_vec;
        self.vertice3 += move_vec;
    }
    pub fn move_to(&mut self, move_vec: Vec3) {
        let original_origin = self.origin;
        self.origin = move_vec;

        // moves vertice to position of new origin shifted by offset between origin and vertex position
        self.vertice1 = self.origin + (self.vertice1 - original_origin);
        self.vertice2 = self.origin + (self.vertice2 - original_origin);
        self.vertice3 = self.origin + (self.vertice3 - original_origin);
    }
    /* Subdivides triangle into two "level" times
     * 
     * Example of a singular subdivision:
     *
     *        2
     *       /|\
     *      / | \
     *     /  |  \
     *    /   |   \
     *   /    |    \
     * 1/_____m_____\3
     *
     * 1 2 3 represent the original vertices
     * m is the new "midpoint" vertex
     * the new triangles are formed from [1 2 m] and [m 2 3] respectively
     *
     * this method consumes the original triangle
     */
    pub fn subdivide(self, level: u8) -> Vec<Triangle> {
        let mut subdivided: Vec<Triangle> = Vec::new();
        subdivided.push(self);

        for current_level in 0..level {
            let mut current_subdivision: Vec<Triangle> = Vec::with_capacity(current_level as usize * 2);

            for tri in &subdivided {
                let midpoint = midpoint!(tri.vertice2, tri.vertice3);

                current_subdivision.push(
                    Triangle {
                        vertice1: midpoint,
                        vertice2: tri.vertice2,
                        vertice3: tri.vertice1,
                        origin: tri.origin,
                        normal: tri.normal,
                        material: tri.material
                    }
                );
                current_subdivision.push(
                    Triangle {
                        vertice1: midpoint,
                        vertice2: tri.vertice1,
                        vertice3: tri.vertice3,
                        origin: tri.origin,
                        normal: tri.normal,
                        material: tri.material
                    }
                );
            }

            subdivided = current_subdivision;
        }

        subdivided
    }
}

pub struct Plane {
    pub triangles: Vec<Triangle>
}

impl Plane {
    /*
     * Constructs new Plane
     * Returns vector of Triangle
     *
     * Placement of vertices. Normal faces observer
     *
     * 3--------4
     * |        |
     * |        |
     * |        |
     * 1--------2
     */
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3, material: materials::Material) -> Plane {
        let origin = midpoint!(v1, v2, v3, v4);
        let mut triangles: Vec<Triangle> = Vec::with_capacity(2);
        triangles.push(Triangle::new_with_origin(v2, v1, v3, material, origin));
        triangles.push(Triangle::new_with_origin(v2, v3, v4, material, origin));

        Plane {
            triangles,
        }
    }
    pub fn new_to_world(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3, material: materials::Material, world: &mut Vec<Triangle>) -> ObjPointer {
        let plane = Plane::new(v1, v2, v3, v4, material);

        world.reserve(2);
        for tri in plane.triangles {
            world.push(tri);
        }

        ObjPointer {
            ptr: world.len() - 2,
            len: 2
        }
    }
    fn new_with_origin(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3, material: materials::Material, origin: Vec3) -> Plane {
        let mut triangles: Vec<Triangle> = Vec::with_capacity(2);
        triangles.push(Triangle::new_with_origin(v1, v3, v2, material, origin));
        triangles.push(Triangle::new_with_origin(v4, v2, v3, material, origin));

        Plane {
            triangles,
        }
    }
}

pub struct Cuboid {
    pub triangles: Vec<Triangle>,
}

impl Cuboid {
    /* 
     * Constructs new Cuboid
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
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3, v5: Vec3, v6: Vec3, v7: Vec3, v8: Vec3, material: materials::Material) -> Cuboid {
        let origin = midpoint!(v1, v2, v3, v4, v5, v6, v7, v8);
        let mut triangles: Vec<Triangle> = Vec::with_capacity(12);

        for tri in Plane::new_with_origin(v1, v2, v3, v4, material, origin).triangles {
            triangles.push(tri);
        }
        for tri in Plane::new_with_origin(v2, v1, v6, v5, material, origin).triangles {
            triangles.push(tri);
        }
        for tri in Plane::new_with_origin(v5, v1, v7, v3, material, origin).triangles {
            triangles.push(tri);
        }
        for tri in Plane::new_with_origin(v2, v6, v4, v8, material, origin).triangles {
            triangles.push(tri);
        }
        for tri in Plane::new_with_origin(v3, v4, v7, v8, material, origin).triangles {
            triangles.push(tri);
        }
        for tri in Plane::new_with_origin(v6, v5, v8, v7, material, origin).triangles {
            triangles.push(tri);
        }

        Cuboid {
            triangles
        }
    } 
    pub fn new_to_world(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3, v5: Vec3, v6: Vec3, v7: Vec3, v8: Vec3, material: materials::Material, world: &mut Vec<Triangle>) -> ObjPointer {
        let cuboid = Cuboid::new(v1, v2, v3, v4, v5, v6, v7, v8, material);

        world.reserve(12);
        for tri in cuboid.triangles {
            world.push(tri);
        }

        ObjPointer {
            ptr: world.len() - 12,
            len: 12
        }
    }
}

pub struct ObjImport {
    pub triangles: Vec<Triangle>,
}

impl ObjImport {
    /* 
     * Constructs new Custom model from .obj wavefront file.
     * Doesn't auto triangulate, requires mesh to be pre triangulated.
     */
    pub fn new(file_name: &str, material: materials::Material) -> ObjImport {
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut import_file = File::open(file_name).unwrap();
        let mut file_contents = String::new();
        let mut vertices: Vec<Vec3> = Vec::new();

        import_file.read_to_string(&mut file_contents).unwrap();

        for line in file_contents.lines() {
            let vertex_data: Vec<&str> = line.split(' ').collect();

            match vertex_data[0] {
                "v" => {
                    let x = vertex_data[1].parse::<f64>().unwrap();
                    let y = vertex_data[2].parse::<f64>().unwrap();
                    let z = vertex_data[3].parse::<f64>().unwrap();

                    vertices.push(Vec3::new(x, y, z));
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

        let origin = {
            let mut origin = Vec3::empty();

            for tri in &triangles {
                origin += tri.origin;
            }

            origin = origin / triangles.len() as f64;

            origin
        };

        for tri in &mut triangles {
            tri.origin = origin;
        }

        ObjImport {
            triangles,
        }
    }
    pub fn new_to_world(file_name: &str, material: materials::Material, world: &mut Vec<Triangle>) -> ObjPointer {
        let import = ObjImport::new(file_name, material);
        let import_size = import.triangles.len();

        world.reserve(import_size);
        for tri in import.triangles {
            world.push(tri);
        }

        ObjPointer {
            ptr: world.len() - import_size,
            len: import_size
        }
    }
}
