use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};

mod perlin;
use perlin::Perlin;

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
    let grid_size: usize = 50;
    let half_grid_size = (grid_size / 2) as i16;
    let treshold = 0.1;

    // let perlin = Perlin::new();

    let circles = vec![(-200.0, 0.0, 30.0), (200.0, 0.0, 25.0)];

    let f = move |x: f32, y: f32| {
        let mut out = 0.0;
        for (xi, yi, ci) in circles.iter() {
            out += ci * ci / ((x - xi) * (x - xi) + (y - yi) * (y - yi));
        }
        out
    };

    let mut positions = Vec::with_capacity(grid_size.pow(2) as usize);
    let mut values = Vec::with_capacity(grid_size.pow(2) as usize);
    let mut values_normalize = Vec::with_capacity(grid_size.pow(2) as usize);

    for y in (-half_grid_size..half_grid_size).rev() {
        for x in -half_grid_size..half_grid_size {
            let position =
                (Vec3::new(x as f32, y as f32, 0.0) + Vec3::new(0.5, 0.5, 0.0)) * sprite_size * 5.0;
            // let value = perlin.turb(
            //     &Vec3::new(
            //         x as f32 / half_grid_size as f32,
            //         y as f32 / half_grid_size as f32,
            //         0.0,
            //     ),
            //     2,
            // );
            let value = f(position[0], position[1]);

            positions.push(position);
            values.push(value);
            values_normalize.push(value > treshold);
        }
    }

    let mut vertices = vec![];
    let mut indices = vec![];

    for i in 0..(grid_size - 1) {
        for j in 0..(grid_size - 1) {
            let a = j + i * grid_size;
            let b = j + i * grid_size + 1;
            let c = j + (i + 1) * grid_size + 1;
            let d = j + (i + 1) * grid_size;

            let a_val = values_normalize[a];
            let b_val = values_normalize[b];
            let c_val = values_normalize[c];
            let d_val = values_normalize[d];

            let mut iso_value = 0;
            iso_value |= (a_val as u8) << 3;
            iso_value |= (b_val as u8) << 2;
            iso_value |= (c_val as u8) << 1;
            iso_value |= d_val as u8;

            match iso_value {
                0 => {}
                1 => {
                    corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        c,
                        d,
                        a,
                    );
                }
                2 => {
                    corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        b,
                        c,
                        d,
                    );
                }
                4 => {
                    corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        a,
                        b,
                        c,
                    );
                }
                8 => {
                    corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        d,
                        a,
                        b,
                    );
                }

                7 => {
                    no_corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        a,
                        b,
                        c,
                        d,
                    );
                }
                11 => {
                    no_corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        b,
                        c,
                        d,
                        a,
                    );
                }
                13 => {
                    no_corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        c,
                        d,
                        a,
                        b,
                    );
                }
                14 => {
                    no_corner(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        d,
                        a,
                        b,
                        c,
                    );
                }

                3 => {
                    split(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        a,
                        b,
                        c,
                        d,
                    );
                }
                6 => {
                    split(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        d,
                        a,
                        b,
                        c,
                    );
                }
                9 => {
                    split(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        b,
                        c,
                        d,
                        a,
                    );
                }
                12 => {
                    split(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        c,
                        d,
                        a,
                        b,
                    );
                }

                5 => {
                    diagonal(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        a,
                        b,
                        c,
                        d,
                    );
                }
                10 => {
                    diagonal(
                        &mut vertices,
                        &mut indices,
                        &mut positions,
                        &mut values,
                        b,
                        c,
                        d,
                        a,
                    );
                }

                15 => {
                    square(&mut vertices, &mut indices, &mut positions, a, b, c, d);
                }

                _ => unreachable!(),
            }
        }
    }

    for (p, v) in positions.iter().zip(values_normalize.iter()) {
        let color = if *v { Color::BLACK } else { Color::WHITE };

        commands.spawn_bundle(SpriteBundle {
            material: color_materials.add(color.into()),
            sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
            transform: Transform::from_translation(*p),
            ..Default::default()
        });
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // vec![[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        vertices,
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        // vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
        vec![[0.0, 0.0, 1.0]; indices.len()],
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        // vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
        vec![[0.0, 0.0]; indices.len()],
    );
    // mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh.set_indices(Some(Indices::U32(indices)));

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

fn corner(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    points: &Vec<Vec3>,
    values: &Vec<f32>,
    p1: usize,
    p2: usize,
    p3: usize,
) {
    let pos_1 = points[p1];
    let pos_2 = points[p2];
    let pos_3 = points[p3];

    let val_1 = values[p1];
    let val_2 = values[p2];
    let val_3 = values[p3];

    let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
    let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

    vertices.extend([
        intersection_2.as_ref(),
        pos_2.as_ref(),
        intersection_1.as_ref(),
    ]);
    let last_index = indices.len() as u32;
    indices.extend([last_index, last_index + 1, last_index + 2]);
}

fn no_corner(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    points: &Vec<Vec3>,
    values: &Vec<f32>,
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    let pos_1 = points[p1];
    let pos_2 = points[p2];
    let pos_3 = points[p3];
    let pos_4 = points[p4];

    let val_1 = values[p1];
    let val_2 = values[p2];
    let val_4 = values[p4];

    let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
    let intersection_2 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

    vertices.extend([
        intersection_2.as_ref(),
        pos_4.as_ref(),
        pos_3.as_ref(),
        intersection_1.as_ref(),
        intersection_2.as_ref(),
        pos_3.as_ref(),
        pos_2.as_ref(),
        intersection_1.as_ref(),
        pos_3.as_ref(),
    ]);
    let last_index = indices.len() as u32;
    indices.extend([
        last_index,
        last_index + 1,
        last_index + 2,
        last_index + 3,
        last_index + 4,
        last_index + 5,
        last_index + 6,
        last_index + 7,
        last_index + 8,
    ]);
}

fn split(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    points: &Vec<Vec3>,
    values: &Vec<f32>,
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    let pos_1 = points[p1];
    let pos_2 = points[p2];
    let pos_3 = points[p3];
    let pos_4 = points[p4];

    let val_1 = values[p1];
    let val_2 = values[p2];
    let val_3 = values[p3];
    let val_4 = values[p4];

    let intersection_1 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));
    let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));

    vertices.extend([
        intersection_1.as_ref(),
        pos_4.as_ref(),
        pos_3.as_ref(),
        intersection_2.as_ref(),
        intersection_1.as_ref(),
        pos_3.as_ref(),
    ]);
    let last_index = indices.len() as u32;
    indices.extend([
        last_index,
        last_index + 1,
        last_index + 2,
        last_index + 3,
        last_index + 4,
        last_index + 5,
    ]);
}

fn diagonal(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    points: &Vec<Vec3>,
    values: &Vec<f32>,
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    let pos_1 = points[p1];
    let pos_2 = points[p2];
    let pos_3 = points[p3];
    let pos_4 = points[p4];

    let val_1 = values[p1];
    let val_2 = values[p2];
    let val_3 = values[p3];
    let val_4 = values[p4];

    let intersection_1 = pos_1.lerp(pos_2, val_2 / (val_1 + val_2));
    let intersection_2 = pos_2.lerp(pos_3, val_3 / (val_2 + val_3));
    let intersection_3 = pos_3.lerp(pos_4, val_4 / (val_3 + val_4));
    let intersection_4 = pos_1.lerp(pos_4, val_4 / (val_1 + val_4));

    vertices.extend([
        intersection_4.as_ref(),
        pos_4.as_ref(),
        intersection_3.as_ref(),
        intersection_1.as_ref(),
        intersection_2.as_ref(),
        pos_2.as_ref(),
    ]);
    let last_index = indices.len() as u32;
    indices.extend([
        last_index,
        last_index + 1,
        last_index + 2,
        last_index + 3,
        last_index + 4,
        last_index + 5,
    ]);
}

fn square(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    points: &Vec<Vec3>,
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    let pos_1 = points[p1];
    let pos_2 = points[p2];
    let pos_3 = points[p3];
    let pos_4 = points[p4];

    vertices.extend([
        pos_1.as_ref(),
        pos_4.as_ref(),
        pos_3.as_ref(),
        pos_2.as_ref(),
        pos_1.as_ref(),
        pos_3.as_ref(),
    ]);
    let last_index = indices.len() as u32;
    indices.extend([
        last_index,
        last_index + 1,
        last_index + 2,
        last_index + 3,
        last_index + 4,
        last_index + 5,
    ]);
}
