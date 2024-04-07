use bevy::prelude::*;
use crate::config::*;
use crate::common::draw_rectable;


pub fn setup_board(commands: Commands) {
    draw_rectable(commands, 0., 0., DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);
}
