use crate::path_tracer::{
    vec3::Vec3,
    geometry::Triangle
};

pub struct BVHNode {
    aabb_min: Vec3,
    aabb_max: Vec3,
    left_node: usize,
    first_tri_idx: usize,
    tri_count: usize,
}

impl BVHNode {
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/*
 * Needs to be flattened to be passed to the gpu.
 * Can't use vectors on the gpu.
 */
pub struct BVH {
    nodes: Vec<BVHNode>,
    tri_indices: Vec<usize>
}

impl BVH {
    pub fn new(tris: &Vec<Triangle>) -> Self {
        let root_node_idx = 0;
        let mut nodes_used = 1;
        let mut bvh_nodes: Vec<BVHNode> = Vec::with_capacity(tris.len() * 2 - 1);
        let mut tri_indices: Vec<usize> = (0..tris.len()).collect();

        let root = &mut bvh_nodes[root_node_idx];
        root.left_node = 0;
        root.first_tri_idx = 0;
        root.tri_count = tris.len();
        update_node_bounds(root_node_idx, &mut bvh_nodes, tris);
        subdivide(root_node_idx, &mut nodes_used, &mut bvh_nodes, tris, &mut tri_indices);

        BVH {
            nodes: bvh_nodes,
            tri_indices
        }
    }
}

fn update_node_bounds(node_idx: usize, bvh_nodes: &mut Vec<BVHNode>, tris: &Vec<Triangle>) {
    let node = &mut bvh_nodes[node_idx];
    node.aabb_min = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
    node.aabb_max = Vec3::new(f64::MIN, f64::MIN, f64::MIN);

    let first = node.first_tri_idx;
    for i in 0..node.tri_count {
        let leaf_tri = tris[first + i];
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
    let j = i + bvh_nodes[node_idx].tri_count - 1;
    while i <= j {
        if tris[tri_indices[i]].origin[axis] < split_pos {
            i += 1;
        } else {
            tri_indices.swap(i, j - 1);
        }
    }

    let left_count = i - bvh_nodes[node_idx].first_tri_idx;
    if left_count == 0 || left_count == bvh_nodes[node_idx].tri_count { return }

    *nodes_used += 1;
    let left_child_idx = *nodes_used;
    *nodes_used += 1;
    let right_child_idx = *nodes_used;

    bvh_nodes[left_child_idx].first_tri_idx = bvh_nodes[node_idx].first_tri_idx;
    bvh_nodes[left_child_idx].tri_count = left_count;
    bvh_nodes[right_child_idx].first_tri_idx = i;
    bvh_nodes[right_child_idx].tri_count = bvh_nodes[node_idx].tri_count - left_count;
    bvh_nodes[node_idx].left_node = left_child_idx;
    bvh_nodes[node_idx].tri_count = 0;

    update_node_bounds(left_child_idx, bvh_nodes, tris);
    update_node_bounds(right_child_idx, bvh_nodes, tris);

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
