use bevy::prelude::*;
use crate::config::*;
use crate::common::draw_rectable;


pub fn setup_board(mut commands: Commands) {
    draw_rectable(&mut commands, Vec3{ x: 0., y: 0., z: 0.}, DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);
    draw_rectable(&mut commands, Vec3{ x: 0., y: 0., z: 1.}, SQUARE_SIZE, SQUARE_SIZE, 5., BOARD_COLOR, BORDER_COLOR);
}
