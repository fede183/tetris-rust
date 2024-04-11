use bevy::prelude::*;
use crate::config::*;

pub fn toggle_resolution(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.resolution.set(DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT);
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn draw_rectable(commands: &mut Commands, positions: Vec3, height: f32, wigth: f32, color: Color) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: positions,
            scale: Vec3 {
                x: wigth,
                y: height,
                z: positions.z,
            },
            ..default()
        },
        sprite: Sprite {
            color,
            ..default()
        },
        ..default()
    });
}

pub fn draw_rectable_with_border(commands: &mut Commands, positions: Vec3, height: f32, wigth: f32, border_size: f32, fill_color: Color, border_color: Color) {
    draw_rectable(commands, Vec3 { z: positions.z + 1., ..positions }, height, wigth, fill_color);

    draw_rectable(commands, positions, height + border_size, wigth + border_size, border_color);
}
