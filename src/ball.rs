use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Ball;

#[derive(Debug, Default, Component)]
pub struct Position {
    pub pos: Vec2,
}
#[derive(Debug, Default, Component)]
pub struct Radius {
    pub r: f32,
}
#[derive(Debug, Default, Component)]
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
            vel: Vec2::new(0.1, -0.4),
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(20.0, 0.0),
        })
        .insert(Radius { r: 2.0 })
        .insert(Veclocity {
            vel: Vec2::new(-0.3, 0.1),
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(30.0, 30.0),
        })
        .insert(Radius { r: 3.0 })
        .insert(Veclocity {
            vel: Vec2::new(-0.2, 0.3),
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(25.0, 25.0),
        })
        .insert(Radius { r: 1.4 })
        .insert(Veclocity {
            vel: Vec2::new(0.87, 1.111),
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(22.0, 202.0),
        })
        .insert(Radius { r: 2.1 })
        .insert(Veclocity {
            vel: Vec2::new(0.5, -0.2),
        });
    commands
        .spawn()
        .insert(Ball)
        .insert(Position {
            pos: Vec2::new(20.0, 20.0),
        })
        .insert(Radius { r: 1.2 })
        .insert(Veclocity {
            vel: Vec2::new(-1.1, 0.9),
        });
}

impl Ball {
    pub fn calc(pos: &Vec2, r: f32, x: f32, y: f32) -> f32 {
        r.powi(2) / ((pos.x - x).powi(2) + (pos.y - y).powi(2))
    }
}
