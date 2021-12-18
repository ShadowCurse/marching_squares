use bevy::prelude::*;

mod grid;
use grid::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_balls.system())
        .add_system(update_mesh.system())
        .run();
}

struct Ball {
    pos: Vec2,
    radius: f32,
    velocity: Vec2,
}

impl Ball {
    pub fn calc(&self, x: f32, y: f32) -> f32 {
        self.radius.powi(2) / ((self.pos.x - x).powi(2) + (self.pos.y - y).powi(2))
    }
}

struct Metabals {
    balls: Vec<Ball>,
}

pub struct GridMesh {
    mesh: Handle<Mesh>,
}

fn setup(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standart_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size = 1.0;
    let grid_size = 1000;
    let threshold = 0.1;

    let balls = Metabals {
        balls: vec![
            Ball {
                pos: Vec2::new(-200.0, 0.0),
                radius: 30.0,
                velocity: Vec2::new(1.0, 2.0),
            },
            Ball {
                pos: Vec2::new(200.0, 0.0),
                radius: 10.0,
                velocity: Vec2::new(-5.0, -11.0),
            },
            Ball {
                pos: Vec2::new(100.0, 40.0),
                radius: 15.0,
                velocity: Vec2::new(-3.0, 1.0),
            },
            Ball {
                pos: Vec2::new(20.0, 20.0),
                radius: 20.0,
                velocity: Vec2::new(5.0, -3.0),
            },
        ],
    };

    let f = |x: f32, y: f32| {
        let mut out = 0.0;
        for ball in balls.balls.iter() {
            out += ball.calc(x, y);
        }
        out
    };

    let mut grid = Grid::new(grid_size, sprite_size * 2.);
    let mesh = grid.create_mesh(&f, threshold);

    // for (p, v) in grid.positions.iter().zip(grid.values_normalize.iter()) {
    //     let color = if *v { Color::BLACK } else { Color::WHITE };

    //     commands.spawn_bundle(SpriteBundle {
    //         material: color_materials.add(color.into()),
    //         sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
    //         transform: Transform::from_translation(*p),
    //         ..Default::default()
    //     });
    // }

    commands.insert_resource(grid);
    commands.insert_resource(balls);
    let h = meshes.add(mesh);
    commands.insert_resource(GridMesh { mesh: h.clone() });
    commands.spawn_bundle(PbrBundle {
        material: standart_materials.add(Color::GREEN.into()),
        mesh: h,
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, 0.0)),
        ..Default::default()
    });

    commands.spawn_bundle(SpriteBundle {
        material: color_materials.add(Color::BLUE.into()),
        sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

fn update_balls(mut balls: ResMut<Metabals>) {
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

fn update_mesh(
    mut grid: ResMut<Grid>,
    balls: Res<Metabals>,
    grid_mesh: ResMut<GridMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let f = |x: f32, y: f32| {
        let mut out = 0.0;
        for ball in balls.balls.iter() {
            out += ball.calc(x, y);
        }
        out
    };
    let threshold = 0.1;
    if let Some(m) = meshes.get_mut(&grid_mesh.mesh) {
        *m = grid.create_mesh(&f, threshold);
    }
}
