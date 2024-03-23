use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::classes::config::{DISPLAY_BOARD_WIDTH, DISPLAY_BOARD_HEIGTH};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(DISPLAY_BOARD_WIDTH, DISPLAY_BOARD_HEIGTH)));

    let color = Color::BLACK;

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT to +X_EXTENT.
            0.0,
            0.0,
            0.0,
        ),
        ..default()
    });
}

