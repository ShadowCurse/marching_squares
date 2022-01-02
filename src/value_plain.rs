use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use std::collections::BTreeMap;

pub struct ValuePlain {
    pub width: u32,
    pub height: u32,
    pub positions: Vec<Vec3>,
    pub values: Vec<f32>,
}

impl ValuePlain {
    pub fn new(width: u32, height: u32) -> Self {
        let total_points = width * height;
        let mut positions = Vec::with_capacity(total_points as usize);
        let half_width = (width / 2) as i32;
        let half_height = (height / 2) as i32;
        for y in (-half_height..half_height).rev() {
            for x in -half_width..half_width {
                let position = Vec3::new(x as f32, y as f32, 0.0) + Vec3::new(0.5, 0.5, 0.0);
                positions.push(position);
            }
        }
        let values = vec![0.0; total_points as usize];

        Self {
            width,
            height,
            positions,
            values,
        }
    }

    pub fn update(&mut self, f: &impl Fn(f32, f32) -> f32) {
        for (i, pos) in self.positions.iter().enumerate() {
            self.values[i] = f(pos[0], pos[1]);
        }
    }
}

#[derive(Debug)]
struct CmpVec3(Vec3);
impl CmpVec3 {
    fn new(vec: Vec3) -> Self {
        Self(vec)
    }
}

impl PartialEq<CmpVec3> for CmpVec3 {
    fn eq(&self, other: &CmpVec3) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for CmpVec3 {}

impl PartialOrd<CmpVec3> for CmpVec3 {
    fn partial_cmp(&self, other: &CmpVec3) -> Option<std::cmp::Ordering> {
        if self.eq(&other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            if self.0.x > other.0.x {
                Some(std::cmp::Ordering::Greater)
            } else if self.0.y > other.0.y {
                Some(std::cmp::Ordering::Greater)
            } else if self.0.z > other.0.z {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Less)
            }
        }
    }
}

impl Ord for CmpVec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct ThresholdLayer {
    pub threshold: f32,
    pub normalized_values: Vec<bool>,
}

impl ThresholdLayer {
    pub fn new(width: u32, height: u32, threshold: f32) -> Self {
        Self {
            threshold,
            normalized_values: vec![false; (width * height) as usize],
        }
    }
    pub fn update_values(&mut self, grid: &ValuePlain) {
        for (n, v) in self.normalized_values.iter_mut().zip(grid.values.iter()) {
            *n = v > &self.threshold;
        }
    }

    pub fn update_mesh(
        &mut self,
        plain: &ValuePlain,
        mesh_handle: Handle<Mesh>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        self.update_values(plain);

        let mut vertex_index = BTreeMap::<CmpVec3, u32>::new();
        let mut vertices = vec![];
        let mut indices = vec![];

        let quad_amount = (plain.width) * (plain.height);
        let mut quads = vec![false; quad_amount as usize];

        for j in 0..(plain.height - 1) {
            for i in 0..(plain.width - 1) {
                let a = (i + j * plain.width) as usize;
                if quads[a] {
                    continue;
                }
                let b = (i + j * plain.width + 1) as usize;
                let c = (i + (j + 1) * plain.width + 1) as usize;
                let d = (i + (j + 1) * plain.width) as usize;

                match self.calculate_iso(plain, i, j) {
                    0 => {}
                    1 => {
                        Self::corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            c,
                            d,
                            a,
                        );
                    }
                    2 => {
                        Self::corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            b,
                            c,
                            d,
                        );
                    }
                    4 => {
                        Self::corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            a,
                            b,
                            c,
                        );
                    }
                    8 => {
                        Self::corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            d,
                            a,
                            b,
                        );
                    }

                    7 => {
                        Self::no_corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            a,
                            b,
                            c,
                            d,
                        );
                    }
                    11 => {
                        Self::no_corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            b,
                            c,
                            d,
                            a,
                        );
                    }
                    13 => {
                        Self::no_corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            c,
                            d,
                            a,
                            b,
                        );
                    }
                    14 => {
                        Self::no_corner(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            d,
                            a,
                            b,
                            c,
                        );
                    }

                    3 => {
                        Self::split(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            a,
                            b,
                            c,
                            d,
                        );
                    }
                    6 => {
                        Self::split(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            d,
                            a,
                            b,
                            c,
                        );
                    }
                    9 => {
                        Self::split(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            b,
                            c,
                            d,
                            a,
                        );
                    }
                    12 => {
                        Self::split(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            c,
                            d,
                            a,
                            b,
                        );
                    }

                    5 => {
                        Self::diagonal(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            a,
                            b,
                            c,
                            d,
                        );
                    }
                    10 => {
                        Self::diagonal(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            b,
                            c,
                            d,
                            a,
                        );
                    }
                    15 => {
                        self.square(
                            plain,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            &mut quads,
                            i,
                            j,
                        );
                    }

                    _ => unreachable!(),
                }
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0]; vertices.len()],
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertices.len()]);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(indices)));
        if let Some(m) = meshes.get_mut(&mesh_handle) {
            *m = mesh;
        }
    }

    fn insert_vertices(
        to_insert: [&Vec3; 3],
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
    ) {
        for v in to_insert {
            let cv = CmpVec3::new(*v);
            if let Some(i) = vertex_index.get(&cv) {
                indices.push(*i);
            } else {
                vertices.push(*v.as_ref());
                let i = vertices.len() as u32 - 1;
                indices.push(i);
                vertex_index.insert(cv, i);
            }
        }
    }

    fn corner(
        plain: &ValuePlain,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
    ) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_3 = plain.values[p3];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

        Self::insert_vertices(
            [&intersection_2, &pos_2, &intersection_1],
            vertices,
            indices,
            vertex_index,
        );
    }

    fn no_corner(
        plain: &ValuePlain,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];
        let pos_4 = plain.positions[p4];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_4 = plain.values[p4];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

        Self::insert_vertices(
            [&intersection_2, &pos_4, &pos_3],
            vertices,
            indices,
            vertex_index,
        );
        Self::insert_vertices(
            [&intersection_1, &intersection_2, &pos_3],
            vertices,
            indices,
            vertex_index,
        );
        Self::insert_vertices(
            [&pos_2, &intersection_1, &pos_3],
            vertices,
            indices,
            vertex_index,
        );
    }

    fn split(
        plain: &ValuePlain,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];
        let pos_4 = plain.positions[p4];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_3 = plain.values[p3];
        let val_4 = plain.values[p4];

        let intersection_1 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

        Self::insert_vertices(
            [&intersection_1, &pos_4, &pos_3],
            vertices,
            indices,
            vertex_index,
        );
        Self::insert_vertices(
            [&intersection_2, &intersection_1, &pos_3],
            vertices,
            indices,
            vertex_index,
        );
    }

    fn diagonal(
        plain: &ValuePlain,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];
        let pos_4 = plain.positions[p4];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_3 = plain.values[p3];
        let val_4 = plain.values[p4];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));
        let intersection_3 = pos_3.lerp(pos_4, val_4 / (val_3 + val_4));
        let intersection_4 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

        Self::insert_vertices(
            [&intersection_4, &pos_4, &intersection_3],
            vertices,
            indices,
            vertex_index,
        );
        Self::insert_vertices(
            [&intersection_1, &intersection_2, &pos_2],
            vertices,
            indices,
            vertex_index,
        );
    }

    fn square(
        &self,
        plain: &ValuePlain,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        quads: &mut Vec<bool>,
        i: u32,
        j: u32,
    ) {
        let mut width = 1;
        let mut height = 1;

        let mut new_i = i + 1;
        let mut next_iso = self.calculate_iso(plain, new_i, j);
        while next_iso == 15
            && new_i < (plain.width - 2)
            && !quads[(new_i + j * plain.width) as usize]
        {
            width += 1;
            new_i += 1;
            next_iso = self.calculate_iso(plain, new_i, j);
        }
        let mut new_j = j + 1;
        next_iso = self.calculate_iso(plain, i, new_j);
        while next_iso == 15 && new_j < (plain.height - 2) {
            new_i = i;
            while next_iso == 15
                && !quads[(new_i + new_j * plain.width) as usize]
                && new_i < i + width
            {
                new_i += 1;
                next_iso = self.calculate_iso(plain, new_i, new_j);
            }
            if new_i - i != width {
                break;
            }
            new_j += 1;
            height += 1;
            next_iso = self.calculate_iso(plain, i, new_j);
        }
        for h in 0..height {
            for w in 0..width {
                quads[(i + w + (j + h) * plain.width) as usize] = true;
            }
        }

        let p1 = (i + j * plain.width) as usize;
        let p2 = (i + width + j * plain.width) as usize;
        let p3 = (i + width + (j + height) * plain.width) as usize;
        let p4 = (i + (j + height) * plain.width) as usize;

        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];
        let pos_4 = plain.positions[p4];

        Self::insert_vertices([&pos_1, &pos_4, &pos_3], vertices, indices, vertex_index);
        Self::insert_vertices([&pos_2, &pos_1, &pos_3], vertices, indices, vertex_index);
    }

    fn calculate_iso(&self, plain: &ValuePlain, i: u32, j: u32) -> u8 {
        if i >= plain.width - 2 || j > plain.height - 2 {
            return 0;
        }
        let a = (i + j * plain.width) as usize;
        let b = (i + j * plain.width + 1) as usize;
        let c = (i + (j + 1) * plain.width + 1) as usize;
        let d = (i + (j + 1) * plain.width) as usize;

        let a_val = self.normalized_values[a];
        let b_val = self.normalized_values[b];
        let c_val = self.normalized_values[c];
        let d_val = self.normalized_values[d];

        let mut iso_value = 0;
        iso_value |= (a_val as u8) << 3;
        iso_value |= (b_val as u8) << 2;
        iso_value |= (c_val as u8) << 1;
        iso_value |= d_val as u8;
        iso_value
    }
}
