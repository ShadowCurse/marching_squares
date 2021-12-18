use bevy::math::Vec3;
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};

pub struct Grid {
    pub size: u32,
    pub positions: Vec<Vec3>,
    pub values: Vec<f32>,
    pub values_normalize: Vec<bool>,
}

impl Grid {
    pub fn new(size: u32, spacing: f32) -> Self {
        let half_size = (size / 2) as i32;

        let mut positions = Vec::with_capacity(size.pow(2) as usize);
        for y in (-half_size..half_size).rev() {
            for x in -half_size..half_size {
                let position =
                    (Vec3::new(x as f32, y as f32, 0.0) + Vec3::new(0.5, 0.5, 0.0)) * spacing;

                positions.push(position);
            }
        }

        let values = vec![0.0; size.pow(2) as usize];
        let values_normalize = vec![false; size.pow(2) as usize];

        Self {
            size,
            positions,
            values,
            values_normalize,
        }
    }

    pub fn create_mesh(&mut self, f: &impl Fn(f32, f32) -> f32, threshold: f32) -> Mesh {
        for (i, pos) in self.positions.iter().enumerate() {
            let val = f(pos[0], pos[1]);
            self.values[i] = val;
            self.values_normalize[i] = val > threshold;
        }

        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 0..(self.size - 1) {
            for j in 0..(self.size - 1) {
                let a = (j + i * self.size) as usize;
                let b = (j + i * self.size + 1) as usize;
                let c = (j + (i + 1) * self.size + 1) as usize;
                let d = (j + (i + 1) * self.size) as usize;

                let a_val = self.values_normalize[a];
                let b_val = self.values_normalize[b];
                let c_val = self.values_normalize[c];
                let d_val = self.values_normalize[d];

                let mut iso_value = 0;
                iso_value |= (a_val as u8) << 3;
                iso_value |= (b_val as u8) << 2;
                iso_value |= (c_val as u8) << 1;
                iso_value |= d_val as u8;

                match iso_value {
                    0 => {}
                    1 => {
                        self.corner(&mut vertices, &mut indices, c, d, a);
                    }
                    2 => {
                        self.corner(&mut vertices, &mut indices, b, c, d);
                    }
                    4 => {
                        self.corner(&mut vertices, &mut indices, a, b, c);
                    }
                    8 => {
                        self.corner(&mut vertices, &mut indices, d, a, b);
                    }

                    7 => {
                        self.no_corner(&mut vertices, &mut indices, a, b, c, d);
                    }
                    11 => {
                        self.no_corner(&mut vertices, &mut indices, b, c, d, a);
                    }
                    13 => {
                        self.no_corner(&mut vertices, &mut indices, c, d, a, b);
                    }
                    14 => {
                        self.no_corner(&mut vertices, &mut indices, d, a, b, c);
                    }

                    3 => {
                        self.split(&mut vertices, &mut indices, a, b, c, d);
                    }
                    6 => {
                        self.split(&mut vertices, &mut indices, d, a, b, c);
                    }
                    9 => {
                        self.split(&mut vertices, &mut indices, b, c, d, a);
                    }
                    12 => {
                        self.split(&mut vertices, &mut indices, c, d, a, b);
                    }

                    5 => {
                        self.diagonal(&mut vertices, &mut indices, a, b, c, d);
                    }
                    10 => {
                        self.diagonal(&mut vertices, &mut indices, b, c, d, a);
                    }

                    15 => {
                        self.square(&mut vertices, &mut indices, a, b, c, d);
                    }

                    _ => unreachable!(),
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            // vec![[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            vertices,
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            // vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
            vec![[0.0, 0.0, 1.0]; indices.len()],
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_UV_0,
            // vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
            vec![[0.0, 0.0]; indices.len()],
        );
        // mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    fn corner(
        &self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        p1: usize,
        p2: usize,
        p3: usize,
    ) {
        let pos_1 = self.positions[p1];
        let pos_2 = self.positions[p2];
        let pos_3 = self.positions[p3];

        let val_1 = self.values[p1];
        let val_2 = self.values[p2];
        let val_3 = self.values[p3];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

        vertices.extend([
            intersection_2.as_ref(),
            pos_2.as_ref(),
            intersection_1.as_ref(),
        ]);
        let last_index = indices.len() as u32;
        indices.extend([last_index, last_index + 1, last_index + 2]);
    }

    fn no_corner(
        &self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = self.positions[p1];
        let pos_2 = self.positions[p2];
        let pos_3 = self.positions[p3];
        let pos_4 = self.positions[p4];

        let val_1 = self.values[p1];
        let val_2 = self.values[p2];
        let val_4 = self.values[p4];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

        vertices.extend([
            intersection_2.as_ref(),
            pos_4.as_ref(),
            pos_3.as_ref(),
            intersection_1.as_ref(),
            intersection_2.as_ref(),
            pos_3.as_ref(),
            pos_2.as_ref(),
            intersection_1.as_ref(),
            pos_3.as_ref(),
        ]);
        let last_index = indices.len() as u32;
        indices.extend([
            last_index,
            last_index + 1,
            last_index + 2,
            last_index + 3,
            last_index + 4,
            last_index + 5,
            last_index + 6,
            last_index + 7,
            last_index + 8,
        ]);
    }

    fn split(
        &self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = self.positions[p1];
        let pos_2 = self.positions[p2];
        let pos_3 = self.positions[p3];
        let pos_4 = self.positions[p4];

        let val_1 = self.values[p1];
        let val_2 = self.values[p2];
        let val_3 = self.values[p3];
        let val_4 = self.values[p4];

        let intersection_1 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

        vertices.extend([
            intersection_1.as_ref(),
            pos_4.as_ref(),
            pos_3.as_ref(),
            intersection_2.as_ref(),
            intersection_1.as_ref(),
            pos_3.as_ref(),
        ]);
        let last_index = indices.len() as u32;
        indices.extend([
            last_index,
            last_index + 1,
            last_index + 2,
            last_index + 3,
            last_index + 4,
            last_index + 5,
        ]);
    }

    fn diagonal(
        &self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = self.positions[p1];
        let pos_2 = self.positions[p2];
        let pos_3 = self.positions[p3];
        let pos_4 = self.positions[p4];

        let val_1 = self.values[p1];
        let val_2 = self.values[p2];
        let val_3 = self.values[p3];
        let val_4 = self.values[p4];

        let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
        let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));
        let intersection_3 = pos_3.lerp(pos_4, val_4 / (val_3 + val_4));
        let intersection_4 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

        vertices.extend([
            intersection_4.as_ref(),
            pos_4.as_ref(),
            intersection_3.as_ref(),
            intersection_1.as_ref(),
            intersection_2.as_ref(),
            pos_2.as_ref(),
        ]);
        let last_index = indices.len() as u32;
        indices.extend([
            last_index,
            last_index + 1,
            last_index + 2,
            last_index + 3,
            last_index + 4,
            last_index + 5,
        ]);
    }

    fn square(
        &self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = self.positions[p1];
        let pos_2 = self.positions[p2];
        let pos_3 = self.positions[p3];
        let pos_4 = self.positions[p4];

        vertices.extend([
            pos_1.as_ref(),
            pos_4.as_ref(),
            pos_3.as_ref(),
            pos_2.as_ref(),
            pos_1.as_ref(),
            pos_3.as_ref(),
        ]);
        let last_index = indices.len() as u32;
        indices.extend([
            last_index,
            last_index + 1,
            last_index + 2,
            last_index + 3,
            last_index + 4,
            last_index + 5,
        ]);
    }
}
