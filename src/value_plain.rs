use bevy::prelude::*;

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
