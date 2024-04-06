use bevy::prelude::*;
use crate::config::*;


pub fn setup_board(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            scale: Vec3 {
                x: DISPLAY_BOARD_WIGTH,
                y: DISPLAY_BOARD_HEIGHT,
                z: 1.,
            },
            ..default()
        },
        sprite: Sprite {
            color: BOARD_COLOR,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            scale: Vec3 {
                x: DISPLAY_BORDER_WIGTH,
                y: DISPLAY_BORDER_HEIGHT,
                z: 0.,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
}
