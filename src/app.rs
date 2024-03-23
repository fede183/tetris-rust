use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::classes::config::{DISPLAY_BOARD_WIDTH, DISPLAY_BOARD_HEIGTH, DISPLAY_BOARD_X, DISPLAY_BOARD_Y, DISPLAY_NEXT_PIECE_BLOCK_WIDTH, DISPLAY_NEXT_PIECE_BLOCK_HEIGTH, 
DISPLAY_NEXT_PIECE_BLOCK_POSITION_X, DISPLAY_NEXT_PIECE_BLOCK_POSITION_Y};


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let color = Color::BLACK;

    let board_shape = Mesh2dHandle(meshes.add(Rectangle::new(DISPLAY_BOARD_WIDTH, DISPLAY_BOARD_HEIGTH)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: board_shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT to +X_EXTENT.
            DISPLAY_BOARD_X,
            DISPLAY_BOARD_Y,
            0.0,
        ),
        ..default()
    });

    let next_piece_block_shape = Mesh2dHandle(meshes.add(Rectangle::new(DISPLAY_NEXT_PIECE_BLOCK_WIDTH, DISPLAY_NEXT_PIECE_BLOCK_HEIGTH)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: next_piece_block_shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            // Distribute shapes from -X_EXTENT to +X_EXTENT.
            DISPLAY_NEXT_PIECE_BLOCK_POSITION_X,
            DISPLAY_NEXT_PIECE_BLOCK_POSITION_Y,
            0.0,
        ),
        ..default()
    });
}

