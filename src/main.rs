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
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
