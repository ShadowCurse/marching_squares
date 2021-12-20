use bevy::prelude::*;

mod grid;
mod metaballs;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(metaballs::setup.system())
        .add_startup_system(grid::setup.system())
        .add_system(metaballs::update_balls.system())
        .add_system(grid::update_mesh.system())
        .run();
}

fn setup(
    mut commands: Commands,
    // mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // let sprite_size = 5.0;

    // commands.spawn_bundle(SpriteBundle {
    //     material: color_materials.add(Color::BLUE.into()),
    //     sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     ..Default::default()
    // });
}
