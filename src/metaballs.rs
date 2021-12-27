use crate::grid::Grid;
use bevy::prelude::*;

pub struct Ball;
pub struct Position {
    pub pos: Vec2,
}
pub struct Radius {
    pub r: f32,
}
pub struct Veclocity {
    pub vel: Vec2,
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(0.0, 0.0),
        })
        .insert(Radius { r: 5.0 })
        .insert(Veclocity {
            vel: Vec2::new(0.05, 0.02),
        });
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert(Position {
    //         pos: Vec2::new(20.0, 0.0),
    //     })
    //     .insert(Radius { r: 2.0 })
    //     .insert(Veclocity {
    //         vel: Vec2::new(-0.03, 0.01),
    //     });
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert(Position {
    //         pos: Vec2::new(20.0, 20.0),
    //     })
    //     .insert(Radius { r: 3.0 })
    //     .insert(Veclocity {
    //         vel: Vec2::new(-0.02, 0.03),
    //     });
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert(Position {
    //         pos: Vec2::new(25.0, 25.0),
    //     })
    //     .insert(Radius { r: 25.0 })
    //     .insert(Veclocity {
    //         vel: Vec2::new(4.0, 3.0),
    //     });
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert(Position {
    //         pos: Vec2::new(22.0, 202.0),
    //     })
    //     .insert(Radius { r: 22.0 })
    //     .insert(Veclocity {
    //         vel: Vec2::new(5.0, -6.0),
    //     });
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert(Position {
    //         pos: Vec2::new(20.0, 20.0),
    //     })
    //     .insert(Radius { r: 10.0 })
    //     .insert(Veclocity {
    //         vel: Vec2::new(-3.0, 3.0),
    //     });
}

pub fn update_balls(grid: Res<Grid>, mut q: Query<(&mut Position, &mut Veclocity), With<Ball>>) {
    let half_width = grid.width as f32 * grid.spacing * 0.5;
    let half_height = grid.height as f32 * grid.spacing * 0.5;
    for (mut pos, mut vel) in q.iter_mut() {
        pos.pos += vel.vel;
        if pos.pos.x > half_width || pos.pos.x < -half_width {
            vel.vel.x *= -1.0;
        }
        if pos.pos.y > half_height || pos.pos.y < -half_height {
            vel.vel.y *= -1.0;
        }
    }
}

impl Ball {
    pub fn calc(pos: &Vec2, r: f32, x: f32, y: f32) -> f32 {
        r.powi(2) / ((pos.x - x).powi(2) + (pos.y - y).powi(2))
    }
}
