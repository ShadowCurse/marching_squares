use bevy::prelude::*;

pub struct Ball {
    pos: Vec2,
    radius: f32,
    velocity: Vec2,
}

impl Ball {
    pub fn calc(&self, x: f32, y: f32) -> f32 {
        self.radius.powi(2) / ((self.pos.x - x).powi(2) + (self.pos.y - y).powi(2))
    }
}

pub struct Metabals {
    balls: Vec<Ball>,
}

impl Metabals {
    pub fn calc(&self, x: f32, y: f32) -> f32 {
        self.balls.iter().fold(0.0, |sum, ball| { sum + ball.calc(x, y) })
    }
}

pub fn update_balls(mut balls: ResMut<Metabals>) {
    for b in balls.balls.iter_mut() {
        b.pos += b.velocity;
        if b.pos.x > 200.0 || b.pos.x < -200.0 {
            b.velocity.x *= -1.0;
        }
        if b.pos.y > 200.0 || b.pos.y < -200.0 {
            b.velocity.y *= -1.0;
        }
    }
}
