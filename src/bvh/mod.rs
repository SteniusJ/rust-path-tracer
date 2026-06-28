use crate::{
    vec3::Vec3,
    geometry::Triangle,
    ray::Ray,
    hitable::HitRecord,
    util::{min_f64, max_f64}
};

#[derive(Clone, Copy)]
pub struct BVHNode {
    aabb_min: Vec3,
    aabb_max: Vec3,
    left_node: usize,
    first_tri_idx: usize,
    tri_count: usize,
}

impl BVHNode {
    pub fn empty() -> Self {
        Self {
            aabb_min: Vec3::empty(),
            aabb_max: Vec3::empty(),
            left_node: 0,
            first_tri_idx: 0,
            tri_count: 0
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

pub struct BVH<'a> {
    nodes: &'a[BVHNode],
    tri_indices: &'a[usize],
    tris: &'a[Triangle]
}

impl<'a> BVH<'a> {
    pub fn new(nodes: &'a[BVHNode], tri_indices: &'a[usize], tris: &'a[Triangle]) -> Self {
        BVH { nodes, tri_indices, tris }
    }
    pub fn intersect(&self, ray: &Ray, hit_rec: &mut HitRecord) -> bool {
        /* Maximum stack size.
         * 100 should be overkill for any reasonable scene.
         */
        static STACK_SIZE: usize = 20;
        let mut stack = [0; STACK_SIZE];
        let mut stack_top = 0_i64;

        let mut closest_t = f64::MAX;
        let mut temp_rec = HitRecord::empty();
        let mut hit = false;

        stack[stack_top as usize] = 0;
        stack_top += 1;

        while stack_top > 0 {
            stack_top -= 1;
            let node_idx = stack[stack_top as usize];
            let node = self.nodes[node_idx];

            if !intersect_aabb(ray, closest_t, node.aabb_min, node.aabb_max) { continue; }

            if node.is_leaf() {
                for i in 0..node.tri_count {
                    let tri_idx = self.tri_indices[node.first_tri_idx + i];
                    if self.tris[tri_idx].hit(ray, 0.0001, closest_t, &mut temp_rec) {
                        hit = true;
                        if closest_t > temp_rec.t {
                            closest_t = temp_rec.t;
                            *hit_rec = temp_rec.clone();
                        }
                    }
                }
            } else {
                if stack_top < STACK_SIZE as i64 {
                    stack[stack_top as usize] = node.left_node + 1;
                    stack_top += 1;
                }
                if stack_top < STACK_SIZE as i64 {
                    stack[stack_top as usize] = node.left_node;
                stack_top += 1;
                }
            }
        }

        hit
    }
    pub fn evaluate(&self) {
        let mut total_leaves = 0_u32;
        let mut leaf_data: Vec<usize> = Vec::new();

        for node in self.nodes {
            if node.is_leaf() {
                total_leaves += 1;
                leaf_data.push(node.tri_count);
            }
        }

        println!(
"
Total nodes {}
Total leaves {}

printing leaf data:
",
self.nodes.len(),
total_leaves
        );

        for (index, data) in leaf_data.iter().enumerate() {
            println!("leaf {index} contains {data} tris");
        }
    }
}

fn intersect_aabb(ray: &Ray, t: f64, b_min: Vec3, b_max: Vec3) -> bool {
    let tx1 = (b_min.x - ray.origin.x) / ray.direction.x;
    let tx2 = (b_max.x - ray.origin.x) / ray.direction.x;
    let tmin = min_f64(tx1, tx2);
    let tmax = max_f64(tx1, tx2);
    let ty1 = (b_min.y - ray.origin.y) / ray.direction.y;
    let ty2 = (b_max.y - ray.origin.y) / ray.direction.y;
    let tmin = max_f64(tmin, min_f64(ty1, ty2));
    let tmax = min_f64(tmax, max_f64(ty1, ty2));
    let tz1 = (b_min.z - ray.origin.z) / ray.direction.z;
    let tz2 = (b_max.z - ray.origin.z) / ray.direction.z;
    let tmin = max_f64(tmin, min_f64(tz1, tz2));
    let tmax = min_f64(tmax, max_f64(tz1, tz2));

    tmax >= tmin && tmin < t && tmax > 0.0
}

pub fn build_bvh(tris: &Vec<Triangle>) -> (Vec<BVHNode>, Vec<usize>) {
    let root_node_idx = 0;
    let mut nodes_used = 1;
    let mut bvh_nodes: Vec<BVHNode> = Vec::with_capacity(tris.len() * 2 - 1);
    let mut tri_indices: Vec<usize> = (0..tris.len()).collect();

    bvh_nodes.insert(root_node_idx, BVHNode::empty());
    let root = &mut bvh_nodes[root_node_idx];
    root.left_node = 0;
    root.first_tri_idx = 0;
    root.tri_count = tris.len();
    update_node_bounds(root_node_idx, &mut bvh_nodes, &tri_indices, tris);
    subdivide(root_node_idx, &mut nodes_used, &mut bvh_nodes, tris, &mut tri_indices);

    (bvh_nodes, tri_indices)
}

fn update_node_bounds(node_idx: usize, bvh_nodes: &mut Vec<BVHNode>, tri_indices: &Vec<usize>, tris: &Vec<Triangle>) {
    let node = &mut bvh_nodes[node_idx];
    node.aabb_min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
    node.aabb_max = Vec3::new(f64::MIN, f64::MIN, f64::MIN);

    for i in 0..node.tri_count {
        let leaf_tri_idx = tri_indices[node.first_tri_idx + i];
        let leaf_tri = tris[leaf_tri_idx];
        node.aabb_min = fminf(node.aabb_min, leaf_tri.vertice1);
        node.aabb_min = fminf(node.aabb_min, leaf_tri.vertice2);
        node.aabb_min = fminf(node.aabb_min, leaf_tri.vertice3);
        node.aabb_max = fmaxf(node.aabb_max, leaf_tri.vertice1);
        node.aabb_max = fmaxf(node.aabb_max, leaf_tri.vertice2);
        node.aabb_max = fmaxf(node.aabb_max, leaf_tri.vertice3);
    }
}

fn subdivide(node_idx: usize, nodes_used: &mut usize, bvh_nodes: &mut Vec<BVHNode>, tris: &Vec<Triangle>, tri_indices: &mut Vec<usize>) {
    if bvh_nodes[node_idx].tri_count <= 2 { return }

    let extent = bvh_nodes[node_idx].aabb_max - bvh_nodes[node_idx].aabb_min;
    let mut axis = 0;
    if extent.y > extent.x { axis = 1 }
    if extent.z > extent[axis] { axis = 2 }

    let split_pos = bvh_nodes[node_idx].aabb_min[axis] + extent[axis] * 0.5;
    let mut i = bvh_nodes[node_idx].first_tri_idx;
    let mut j = i + bvh_nodes[node_idx].tri_count - 1;
    while i <= j {
        if tris[tri_indices[i]].origin[axis] < split_pos {
            i += 1;
        } else {
            tri_indices.swap(i, j);
            if j == 0 { break } // overflow check, if i == 0 j will overflow without this
            j -= 1
        }
    }

    let left_count = i - bvh_nodes[node_idx].first_tri_idx;
    if left_count == 0 || left_count == bvh_nodes[node_idx].tri_count { return }

    let left_child_idx = *nodes_used;
    *nodes_used += 1;
    let right_child_idx = *nodes_used;
    *nodes_used += 1;

    bvh_nodes.insert(left_child_idx, BVHNode::empty());
    bvh_nodes.insert(right_child_idx, BVHNode::empty());

    bvh_nodes[left_child_idx].first_tri_idx = bvh_nodes[node_idx].first_tri_idx;
    bvh_nodes[left_child_idx].tri_count = left_count;
    bvh_nodes[right_child_idx].first_tri_idx = i;
    bvh_nodes[right_child_idx].tri_count = bvh_nodes[node_idx].tri_count - left_count;
    bvh_nodes[node_idx].left_node = left_child_idx;
    bvh_nodes[node_idx].tri_count = 0;

    update_node_bounds(left_child_idx, bvh_nodes, tri_indices, tris);
    update_node_bounds(right_child_idx, bvh_nodes, tri_indices, tris);

    subdivide(left_child_idx, nodes_used, bvh_nodes, tris, tri_indices);
    subdivide(right_child_idx, nodes_used, bvh_nodes, tris, tri_indices);
}

fn fminf(v1: Vec3, v2: Vec3) -> Vec3 {
    let mut rv = Vec3::empty();
    if v1.x < v2.x {
        rv.x = v1.x;
    } else {
        rv.x = v2.x;
    }

    if v1.y < v2.y {
        rv.y = v1.y;
    } else {
        rv.y = v2.y;
    }

    if v1.z < v2.z {
        rv.z = v1.z;
    } else {
        rv.z = v2.z;
    }

    rv
}

fn fmaxf(v1: Vec3, v2: Vec3) -> Vec3 {
    let mut rv = Vec3::empty();
    if v1.x > v2.x {
        rv.x = v1.x;
    } else {
        rv.x = v2.x;
    }

    if v1.y > v2.y {
        rv.y = v1.y;
    } else {
        rv.y = v2.y;
    }

    if v1.z > v2.z {
        rv.z = v1.z;
    } else {
        rv.z = v2.z;
    }

    rv
}
