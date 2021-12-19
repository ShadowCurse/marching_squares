use bevy::prelude::*;

mod grid;
mod metaballs;

use grid::*;
use metaballs::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_balls.system())
        .add_system(update_mesh.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standart_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size = 5.0;
    let grid_size = 50;
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

    let mut grid = Grid::new(grid_size, sprite_size * 2.0, threshold);
    let mesh = grid.create_mesh(&|x, y| balls.calc(x, y), threshold);

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
