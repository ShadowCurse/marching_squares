use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size = 5.0;
    let half_grid_size = 8;

    for x in -half_grid_size..half_grid_size {
        for y in -half_grid_size..half_grid_size {
            let translation = (Vec3::new(x as f32, y as f32, 0.0)
                + Vec3::new(
                    half_grid_size as f32 / 4.0,
                    half_grid_size as f32 / 4.0,
                    0.0,
                ))
                * sprite_size
                * 10.0;
            // println!("adding sprite with translation: {:#?}", translation);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::GREEN.into()),
                sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
                transform: Transform::from_translation(translation),
                ..Default::default()
            });
        }
    }
}
