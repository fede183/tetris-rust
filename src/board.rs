use bevy::prelude::*;
use crate::config::*;
use crate::common::draw_rectable;


pub fn setup_board(mut commands: Commands) {
    draw_rectable(&mut commands, Vec3{ x: 0., y: 0., z: 0.}, DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);
    for x in 0..10 {
        for y in 0..20 {
            let x_position = SQUARE_SIZE* (x as f32);
            let y_position = SQUARE_SIZE* (y as f32);
            let square_size = SQUARE_SIZE - 5.;
            draw_rectable(&mut commands, Vec3{ x: -DISPLAY_FIRST_BOARD_POSITION_X + x_position, y: DISPLAY_FIRST_BOARD_POSITION_Y - y_position, z: 1.}, square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR);
        }
    }
}

pub fn setup_next_piece_board(mut commands: Commands) {
    draw_rectable(&mut commands, Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.}, DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);
}
