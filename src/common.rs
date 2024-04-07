use bevy::prelude::*;
use crate::config::*;

pub fn toggle_resolution(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.resolution.set(DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT);
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn draw_rectable(commands: &mut Commands, positions: Vec3, height: f32, wigth: f32, border_size: f32, fill_color: Color, border_color: Color) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: positions.x,
                y: positions.y,
                z: positions.z + 1.,
            },
            scale: Vec3 {
                x: wigth,
                y: height,
                z: positions.z + 1.,
            },
            ..default()
        },
        sprite: Sprite {
            color: fill_color,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: positions.x,
                y: positions.y,
                z: positions.z,
            },
            scale: Vec3 {
                x: wigth + border_size,
                y: height + border_size,
                z: positions.z,
            },
            ..default()
        },
        sprite: Sprite {
            color: border_color,
            ..default()
        },
        ..default()
    });
}
