use bevy::prelude::*;

mod grid;
use grid::Grid;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
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

    let circles = vec![(-200.0, 0.0, 30.0), (200.0, 0.0, 25.0)];

    let f = move |x: f32, y: f32| {
        let mut out = 0.0;
        for (xi, yi, ci) in circles.iter() {
            out += ci * ci / ((x - xi) * (x - xi) + (y - yi) * (y - yi));
        }
        out
    };

    let mut grid = Grid::new(grid_size, sprite_size * 2.);
    let mesh = grid.create_mesh(&f, threshold);

    for (p, v) in grid.positions.iter().zip(grid.values_normalize.iter()) {
        let color = if *v { Color::BLACK } else { Color::WHITE };

        commands.spawn_bundle(SpriteBundle {
            material: color_materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
            transform: Transform::from_translation(*p),
            ..Default::default()
        });
    }

    commands.spawn_bundle(PbrBundle {
        material: standart_materials.add(Color::GREEN.into()),
        mesh: meshes.add(mesh), //Mesh::from(shape::Quad { size: (2.0, 2.0).into(), flip: true })),
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
