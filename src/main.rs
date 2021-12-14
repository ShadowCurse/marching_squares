use bevy::prelude::*;

mod perlin;
use perlin::Perlin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size = 3.0;
    let half_grid_size = 2;

    let perlin = Perlin::new();

    for x in -half_grid_size..half_grid_size {
        for y in -half_grid_size..half_grid_size {
            let translation =
                (Vec3::new(x as f32, y as f32, 0.0) + Vec3::new(0.5, 0.5, 0.0)) * sprite_size * 2.0;
            let color = Color::rgb(1.0, 1.0, 1.0)
                * 0.5
                * (1.0
                    + (10.0
                        * perlin.turb(
                            &Vec3::new(
                                x as f32 / half_grid_size as f32,
                                y as f32 / half_grid_size as f32,
                                0.0,
                            ),
                            2,
                        ))
                    .sin());
            println!(
                "Adding point with translation: {:?}, color: {:?}",
                translation, color
            );

            commands.spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
                transform: Transform::from_translation(translation),
                ..Default::default()
            });
        }
    }
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::BLUE.into()),
        sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}
