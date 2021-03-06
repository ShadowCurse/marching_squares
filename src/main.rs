use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

mod ball;
mod marching_squares;
mod threshold_layer;
mod value_plain;

use crate::ball::{Ball, Position, Radius, Veclocity};
use crate::threshold_layer::ThresholdLayer;
use crate::value_plain::ValuePlain;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(69, 69, 69)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(ball::setup)
        .add_startup_system(setup_plain_and_layers)
        .add_system(update_balls)
        .add_system(update_plain)
        .add_system(update_layers)
        .add_system(camera_movement)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(100.0, 0.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update_balls(
    plain: Query<&ValuePlain, With<MetaballsPlain>>,
    mut q: Query<(&mut Position, &mut Veclocity), With<Ball>>,
) {
    if let Some(plain) = plain.iter().next() {
        let half_width = plain.width as f32 * 0.5;
        let half_height = plain.height as f32 * 0.5;
        for (mut pos, mut vel) in q.iter_mut() {
            pos.pos += vel.vel;
            if pos.pos.x > half_width || pos.pos.x < -half_width {
                vel.vel.x *= -1.0;
            }
            if pos.pos.y > half_height || pos.pos.y < -half_height {
                vel.vel.y *= -1.0;
            }
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct MetaballsPlain;

pub fn setup_plain_and_layers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standart_materials: ResMut<Assets<StandardMaterial>>,
) {
    let width = 100;
    let height = 100;

    commands
        .spawn()
        .insert(ValuePlain::new(width, height))
        .insert(MetaballsPlain);

    let thresholds = [0.2, 0.1, 0.05, 0.04, 0.03];
    let colors = [
        Color::rgb_u8(250, 110, 229),
        Color::rgb_u8(198, 95, 194),
        Color::rgb_u8(146, 80, 159),
        Color::rgb_u8(94, 65, 123),
        Color::rgb_u8(42, 50, 88),
    ];

    for (i, (t, c)) in thresholds
        .into_iter()
        .zip(colors.into_iter())
        .rev()
        .enumerate()
    {
        commands
            .spawn_bundle(PbrBundle {
                material: standart_materials.add(c.into()),
                mesh: meshes.add(Mesh::new(PrimitiveTopology::TriangleList)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0 * i as f32)),
                ..Default::default()
            })
            .insert(ThresholdLayer::new(width, height, t));
    }
}

pub fn update_plain(
    mut plain: Query<&mut ValuePlain, With<MetaballsPlain>>,
    balls: Query<(&Position, &Radius), With<Ball>>,
) {
    if let Some(mut plain) = plain.iter_mut().next() {
        plain.update(&|x, y| {
            balls
                .iter()
                .fold(0.0, |sum, (p, r)| sum + Ball::calc(&p.pos, r.r, x, y))
        });
    } else {
        println!("no plane");
    }
}

pub fn update_layers(
    mut meshes: ResMut<Assets<Mesh>>,
    plain: Query<&ValuePlain, With<MetaballsPlain>>,
    mut layers: Query<(&mut ThresholdLayer, &Handle<Mesh>)>,
) {
    if let Some(plain) = plain.iter().next() {
        for (mut l, h) in layers.iter_mut() {
            l.update_mesh(plain, h.clone(), &mut meshes);
        }
    }
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
