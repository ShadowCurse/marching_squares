use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use std::collections::BTreeMap;

use crate::metaballs::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standart_materials: ResMut<Assets<StandardMaterial>>,
) {
    let spacing = 0.5;
    let width = 200;
    let height = 200;

    let grid = Grid::new(width, height, spacing);
    commands.insert_resource(grid);

    let thresholds = [0.2]; //, 0.1, 0.05, 0.04, 0.03];
    let colors = [
        Color::ORANGE,
        // Color::GREEN,
        // Color::BLUE,
        // Color::CYAN,
        // Color::TEAL,
    ];

    for (i, (t, c)) in thresholds
        .into_iter()
        .zip(colors.into_iter())
        .rev()
        .enumerate()
    {
        commands
            .spawn_bundle(PbrBundle {
                material: standart_materials.add(c.into()),
                mesh: meshes.add(Mesh::new(PrimitiveTopology::TriangleList)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0 * i as f32)),
                ..Default::default()
            })
            .insert(GridLayer {
                threshold: t,
                values_normalize: vec![false; (width * height) as usize],
            });
    }
}

pub fn update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut grid: ResMut<Grid>,
    balls: Query<(&Position, &Radius), With<Ball>>,
    mut layers: Query<(&mut GridLayer, &Handle<Mesh>)>,
) {
    grid.update(&|x, y| {
        balls
            .iter()
            .fold(0.0, |sum, (p, r)| sum + Ball::calc(&p.pos, r.r, x, y))
    });
    for (mut l, h) in layers.iter_mut() {
        l.update(&grid, h.clone(), &mut meshes);
    }
}

pub struct GridLayer {
    pub threshold: f32,
    pub values_normalize: Vec<bool>,
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub spacing: f32,
    pub positions: Vec<Vec3>,
    pub values: Vec<f32>,
}

impl Grid {
    pub fn new(width: u32, height: u32, spacing: f32) -> Self {
        let total_points = width * height;
        let mut positions = Vec::with_capacity(total_points as usize);
        let half_width = (width / 2) as i32;
        let half_height = (height / 2) as i32;
        for y in (-half_height..half_height).rev() {
            for x in -half_width..half_width {
                let position =
                    (Vec3::new(x as f32, y as f32, 0.0) + Vec3::new(0.5, 0.5, 0.0)) * spacing;
                positions.push(position);
            }
        }
        let values = vec![0.0; total_points as usize];

        Self {
            width,
            height,
            spacing,
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

impl GridLayer {
    pub fn update(
        &mut self,
        grid: &Grid,
        mesh_handle: Handle<Mesh>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        for (i, val) in grid.values.iter().enumerate() {
            self.values_normalize[i] = val > &self.threshold;
        }

        let mut vertex_index = BTreeMap::<CmpVec3, u32>::new();
        let mut vertices = vec![];
        let mut indices = vec![];

        for i in 0..(grid.height - 1) {
            for j in 0..(grid.width - 1) {
                let a = (j + i * grid.width) as usize;
                let b = (j + i * grid.width + 1) as usize;
                let c = (j + (i + 1) * grid.width + 1) as usize;
                let d = (j + (i + 1) * grid.width) as usize;

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
                        Self::corner(
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                            grid,
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
                        Self::square(
                            grid,
                            &mut vertices,
                            &mut indices,
                            &mut vertex_index,
                            a,
                            b,
                            c,
                            d,
                        );
                    }

                    _ => unreachable!(),
                }
            }
        }
        // println!("vertices: {:#?}", vertices.len());
        // println!("indices: {:#?}", indices.len());
        // println!("vertex_index: {:#?}", vertex_index.len());

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
        grid: &Grid,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
    ) {
        let pos_1 = grid.positions[p1];
        let pos_2 = grid.positions[p2];
        let pos_3 = grid.positions[p3];

        let val_1 = grid.values[p1];
        let val_2 = grid.values[p2];
        let val_3 = grid.values[p3];

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
        grid: &Grid,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = grid.positions[p1];
        let pos_2 = grid.positions[p2];
        let pos_3 = grid.positions[p3];
        let pos_4 = grid.positions[p4];

        let val_1 = grid.values[p1];
        let val_2 = grid.values[p2];
        let val_4 = grid.values[p4];

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
        grid: &Grid,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = grid.positions[p1];
        let pos_2 = grid.positions[p2];
        let pos_3 = grid.positions[p3];
        let pos_4 = grid.positions[p4];

        let val_1 = grid.values[p1];
        let val_2 = grid.values[p2];
        let val_3 = grid.values[p3];
        let val_4 = grid.values[p4];

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
        grid: &Grid,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = grid.positions[p1];
        let pos_2 = grid.positions[p2];
        let pos_3 = grid.positions[p3];
        let pos_4 = grid.positions[p4];

        let val_1 = grid.values[p1];
        let val_2 = grid.values[p2];
        let val_3 = grid.values[p3];
        let val_4 = grid.values[p4];

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
        grid: &Grid,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        vertex_index: &mut BTreeMap<CmpVec3, u32>,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) {
        let pos_1 = grid.positions[p1];
        let pos_2 = grid.positions[p2];
        let pos_3 = grid.positions[p3];
        let pos_4 = grid.positions[p4];

        Self::insert_vertices([&pos_1, &pos_4, &pos_3], vertices, indices, vertex_index);
        Self::insert_vertices([&pos_2, &pos_1, &pos_3], vertices, indices, vertex_index);
    }
}
