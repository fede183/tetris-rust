use bevy::prelude::*;
use crate::{config::*, Piece, PointOnBoard};

pub fn toggle_resolution(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.resolution.set(DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT);
}

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn draw_rectangle(commands: &mut Commands, positions: Vec3, height: f32, wigth: f32, color: Color) {
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

pub fn draw_rectangle_with_border(commands: &mut Commands, positions: Vec3, height: f32, wigth: f32, border_size: f32, fill_color: Color, border_color: Color) {
    draw_rectangle(commands, Vec3 { z: positions.z + 1., ..positions }, height, wigth, fill_color);

    draw_rectangle(commands, positions, height + border_size, wigth + border_size, border_color);
}

fn draw_point_in_next_piece(commands: &mut Commands, point: &PointOnBoard) {
    let x_position = point.x as f32 * SQUARE_SIZE;
    let y_position = point.y as f32 * SQUARE_SIZE;
    draw_rectangle(commands, Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X + x_position, y: DISPLAY_NEXT_PIECE_POSITION_Y + y_position, z: 1.}, SQUARE_SIZE, SQUARE_SIZE, point.color);
}

pub fn draw_next_piece(commands: &mut Commands, piece: &Piece) {
    for point in &piece.0 {
        draw_point_in_next_piece(commands, &point);
    }
}

pub fn draw_cell_on_board(commands: &mut Commands, x: i32, y: i32) {
    let x_position = SQUARE_SIZE* (x as f32);
    let y_position = SQUARE_SIZE* (y as f32);
    let square_size = SQUARE_SIZE - 5.;
    draw_rectangle_with_border(commands, Vec3{ x: -DISPLAY_FIRST_BOARD_POSITION_X + x_position, y: DISPLAY_FIRST_BOARD_POSITION_Y - y_position, z: 1.}, square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR);
}

fn draw_point_in_board(commands: &mut Commands, point: &PointOnBoard) {
    let x_position = point.x as f32 * SQUARE_SIZE;
    let y_position = point.y as f32 * SQUARE_SIZE;
    draw_rectangle(commands, Vec3{ x: -DISPLAY_FIRST_BOARD_POSITION_X + x_position, y: DISPLAY_FIRST_BOARD_POSITION_Y - y_position, z: 2.}, SQUARE_SIZE, SQUARE_SIZE, point.color);
}

pub fn draw_piece_on_board(commands: &mut Commands, piece: &Piece) {
    for point in &piece.0 {
        draw_point_in_board(commands, &point);
    }
}
