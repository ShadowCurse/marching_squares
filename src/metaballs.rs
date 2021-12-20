use bevy::prelude::*;

pub struct Ball;
pub struct Position{
   pub pos: Vec2,
}
pub struct Radius{
    pub r: f32,
}
pub struct Veclocity{
    pub vel: Vec2,
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(-200.0, 0.0) })
        .insert(Radius { r: 30.0 })
        .insert(Veclocity { vel: Vec2::new(1.0, 2.0) });
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(200.0, 0.0) })
        .insert(Radius { r: 20.0 })
        .insert(Veclocity { vel: Vec2::new(2.0, 4.0) });
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(20.0, 20.0) })
        .insert(Radius { r: 10.0 })
        .insert(Veclocity { vel: Vec2::new(3.0, 6.0) });
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(25.0, 25.0) })
        .insert(Radius { r: 25.0 })
        .insert(Veclocity { vel: Vec2::new(4.0, 3.0) });
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(22.0, 202.0) })
        .insert(Radius { r: 22.0 })
        .insert(Veclocity { vel: Vec2::new(5.0, -6.0) });
    commands.spawn()
        .insert(Ball)
        .insert(Position { pos: Vec2::new(20.0, 20.0) })
        .insert(Radius { r: 10.0 })
        .insert(Veclocity { vel: Vec2::new(-3.0, 3.0) });
}

pub fn update_balls(mut q: Query<(&mut Position, &mut Veclocity), With<Ball>>) {
    for (mut pos, mut vel) in q.iter_mut() {
        pos.pos += vel.vel;
        if pos.pos.x > 500.0 || pos.pos.x < -500.0 {
            vel.vel.x *= -1.0;
        }
        if pos.pos.y > 500.0 || pos.pos.y < -500.0 {
            vel.vel.y *= -1.0;
        }
    }
}


// pub struct Ball {
//     pos: Vec2,
//     radius: f32,
//     velocity: Vec2,
// }

impl Ball {
    pub fn calc(pos: &Vec2, r: f32, x: f32, y: f32) -> f32 {
        r.powi(2) / ((pos.x - x).powi(2) + (pos.y - y).powi(2))
    }
}

// pub struct Metabals {
//     balls: Vec<Ball>,
// }

// impl Metabals {
//     pub fn calc(&self, x: f32, y: f32) -> f32 {
//         self.balls.iter().fold(0.0, |sum, ball| { sum + ball.calc(x, y) })
//     }
// }

// pub fn update_balls(mut balls: ResMut<Metabals>) {
//     for b in balls.balls.iter_mut() {
//         b.pos += b.velocity;
//         if b.pos.x > 200.0 || b.pos.x < -200.0 {
//             b.velocity.x *= -1.0;
//         }
//         if b.pos.y > 200.0 || b.pos.y < -200.0 {
//             b.velocity.y *= -1.0;
//         }
//     }
// }
