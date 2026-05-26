use crate::{geometry, materials};

pub struct World {
    geometry: Vec<geometry::Triangle>,
    materials: Vec<materials::Material>,
}

impl World {
    pub fn new() -> Self {
        Self {
            geometry: Vec::new(),
            materials: Vec::new()
        }
    } 
    /*
     * Adds material to world material vector.
     * Returns material pointer.
     */
    pub fn add_material(&mut self, material: materials::Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }
    pub fn add_geometry(&mut self, triangles: Vec<geometry::Triangle>) {
        self.geometry.reserve(triangles.len());
        for tri in triangles {
            self.geometry.push(tri);
        }
    }
    pub fn add_triangle(&mut self, triangle: geometry::Triangle) {
        self.geometry.push(triangle);
    }
}
