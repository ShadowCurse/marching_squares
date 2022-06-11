use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};
use std::collections::BTreeMap;

use crate::value_plain::ValuePlain;
use crate::threshold_layer::ThresholdLayer;

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

impl PartialOrd<Self> for CmpVec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else if self.0.x > other.0.x || self.0.y > other.0.y || self.0.z > other.0.z {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Ord for CmpVec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Default)]
pub struct MarchingSquares {
    vertex_index: BTreeMap<CmpVec3, u32>,
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

impl MarchingSquares {
    pub fn from_plain(plain: &ValuePlain, layer: &ThresholdLayer) -> Mesh {
        Self::default().mesh_from_plain(plain, layer)
    }
    pub fn mesh_from_plain(mut self, plain: &ValuePlain, layer: &ThresholdLayer) -> Mesh {
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

                match layer.calculate_iso(plain, i, j) {
                    0 => {}
                    1 => {
                        self.corner(plain, c, d, a);
                    }
                    2 => {
                        self.corner(plain, b, c, d);
                    }
                    4 => {
                        self.corner(plain, a, b, c);
                    }
                    8 => {
                        self.corner(plain, d, a, b);
                    }

                    7 => {
                        self.no_corner(plain, a, b, c, d);
                    }
                    11 => {
                        self.no_corner(plain, b, c, d, a);
                    }
                    13 => {
                        self.no_corner(plain, c, d, a, b);
                    }
                    14 => {
                        self.no_corner(plain, d, a, b, c);
                    }

                    3 => {
                        self.split(plain, a, b, c, d);
                    }
                    6 => {
                        self.split(plain, d, a, b, c);
                    }
                    9 => {
                        self.split(plain, b, c, d, a);
                    }
                    12 => {
                        self.split(plain, c, d, a, b);
                    }

                    5 => {
                        self.diagonal(plain, a, b, c, d);
                    }
                    10 => {
                        self.diagonal(plain, b, c, d, a);
                    }
                    15 => {
                        self.square(plain, layer, &mut quads, i, j);
                    }

                    _ => unreachable!(),
                }
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0]; self.vertices.len()],
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; self.vertices.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.set_indices(Some(Indices::U32(self.indices)));
        mesh
    }

    fn insert_vertices(&mut self, to_insert: [&Vec3; 3]) {
        for v in to_insert {
            let cv = CmpVec3::new(*v);
            if let Some(i) = self.vertex_index.get(&cv) {
                self.indices.push(*i);
            } else {
                self.vertices.push(*v.as_ref());
                let i = self.vertices.len() as u32 - 1;
                self.indices.push(i);
                self.vertex_index.insert(cv, i);
            }
        }
    }

    fn corner(&mut self, plain: &ValuePlain, p1: usize, p2: usize, p3: usize) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_3 = plain.values[p3];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

        self.insert_vertices([&intersection_2, &pos_2, &intersection_1]);
    }

    fn no_corner(&mut self, plain: &ValuePlain, p1: usize, p2: usize, p3: usize, p4: usize) {
        let pos_1 = plain.positions[p1];
        let pos_2 = plain.positions[p2];
        let pos_3 = plain.positions[p3];
        let pos_4 = plain.positions[p4];

        let val_1 = plain.values[p1];
        let val_2 = plain.values[p2];
        let val_4 = plain.values[p4];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

        self.insert_vertices([&intersection_2, &pos_4, &pos_3]);
        self.insert_vertices([&intersection_1, &intersection_2, &pos_3]);
        self.insert_vertices([&pos_2, &intersection_1, &pos_3]);
    }

    fn split(&mut self, plain: &ValuePlain, p1: usize, p2: usize, p3: usize, p4: usize) {
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

        self.insert_vertices([&intersection_1, &pos_4, &pos_3]);
        self.insert_vertices([&intersection_2, &intersection_1, &pos_3]);
    }

    fn diagonal(&mut self, plain: &ValuePlain, p1: usize, p2: usize, p3: usize, p4: usize) {
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

        self.insert_vertices([&intersection_4, &pos_4, &intersection_3]);
        self.insert_vertices([&intersection_1, &intersection_2, &pos_2]);
    }

    fn square(
        &mut self,
        plain: &ValuePlain,
        layer: &ThresholdLayer,
        quads: &mut Vec<bool>,
        i: u32,
        j: u32,
    ) {
        let mut width = 1;
        let mut height = 1;

        let mut new_i = i + 1;
        let mut next_iso = layer.calculate_iso(plain, new_i, j);
        while next_iso == 15
            && new_i < (plain.width - 2)
            && !quads[(new_i + j * plain.width) as usize]
        {
            width += 1;
            new_i += 1;
            next_iso = layer.calculate_iso(plain, new_i, j);
        }
        let mut new_j = j + 1;
        next_iso = layer.calculate_iso(plain, i, new_j);
        while next_iso == 15 && new_j < (plain.height - 2) {
            new_i = i;
            while next_iso == 15
                && !quads[(new_i + new_j * plain.width) as usize]
                && new_i < i + width
            {
                new_i += 1;
                next_iso = layer.calculate_iso(plain, new_i, new_j);
            }
            if new_i - i != width {
                break;
            }
            new_j += 1;
            height += 1;
            next_iso = layer.calculate_iso(plain, i, new_j);
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

        self.insert_vertices([&pos_1, &pos_4, &pos_3]);
        self.insert_vertices([&pos_2, &pos_1, &pos_3]);
    }
}
