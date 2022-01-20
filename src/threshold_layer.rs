use bevy::prelude::*;
use crate::value_plain::ValuePlain;
use crate::marching_squares::MarchingSquares;

#[derive(Debug, Default, Component)]
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
        let mesh = MarchingSquares::from_plain(plain, self);
        if let Some(m) = meshes.get_mut(&mesh_handle) {
            *m = mesh;
        }
    }

    pub fn calculate_iso(&self, plain: &ValuePlain, i: u32, j: u32) -> u8 {
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
