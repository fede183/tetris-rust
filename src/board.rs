use bevy::prelude::*;
use crate::config::*;


pub fn setup_board(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                ..default()
            },
            scale: Vec3 {
                x: DISPLAY_BOARD_WIGTH,
                y: DISPLAY_BOARD_HEIGHT,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BOARD_COLOR,
            ..default()
        },
        ..default()
    });
}
