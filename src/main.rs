use bevy::prelude::*;

mod grid;
mod metaballs;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(metaballs::setup.system())
        .add_startup_system(grid::setup.system())
        // .add_system(metaballs::update_balls.system())
        .add_system(grid::update_mesh.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(100.0, 0.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
    commands
        .spawn_bundle(LightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            light: Light {
                intensity: 1000.0,
                range: 1000.0,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 50.0),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(10.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<&mut Transform, With<bevy::render::camera::Camera>>,
) {
    for mut transform in q.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.z -= 2.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.z += 2.;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::LControl) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            transform.rotation *= Quat::from_rotation_y(0.1);
        }
        if keyboard_input.pressed(KeyCode::E) {
            transform.rotation *= Quat::from_rotation_y(-0.1);
        }
    }
}
