use bevy::prelude::*;
use crate::{config::*, generate_rectangle};
use crate::common::{generate_rectangle_with_border};
use crate::game::*;
use rand::Rng;

fn get_position_on_board(x: i32, y: i32) -> (f32, f32) {
    let x_position = SQUARE_SIZE* (x as f32);
    let y_position = SQUARE_SIZE* (y as f32);

    (-DISPLAY_FIRST_BOARD_POSITION_X + x_position, DISPLAY_FIRST_BOARD_POSITION_Y - y_position)
}

fn get_position_point_on_board(x: i32, y: i32) -> Vec3 {
    let (x_position, y_position) = get_position_on_board(x, y);

    Vec3 { x: x_position as f32, y: y_position as f32, z: 2.}
}

fn get_position_on_next_piece(x: i32, y: i32) -> (f32, f32) {
    let x_position = SQUARE_SIZE* (x as f32);
    let y_position = SQUARE_SIZE* (y as f32);

    (DISPLAY_NEXT_PIECE_POSITION_X + x_position, DISPLAY_NEXT_PIECE_POSITION_Y + y_position)
}

fn get_position_point_next_piece(x: i32, y: i32) -> Vec3 {
    let (x_position, y_position) = get_position_on_next_piece(x, y);

    Vec3 { x: x_position as f32, y: y_position as f32, z: 2.}
}

fn generate_cell_on_board(x: i32, y: i32) -> [SpriteBundle; 2] {
    let (x_position, y_position) = get_position_on_board(x, y);
    let square_size = SQUARE_SIZE - 5.;
    generate_rectangle_with_border(Vec3{ x: x_position, y: y_position, z: 1.}, square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR)
}

fn spawn_point_in_next_piece(x: i32, y: i32, color: Color) {
    let sprite = generate_rectangle(get_position_point_next_piece(x, y), SQUARE_SIZE, SQUARE_SIZE, color);
}

fn generate_point_in_board(x: i32, y: i32, color: Color) -> PointComponent {
    let (x_position, y_position) = get_position_on_board(x, y);
    let sprite = generate_rectangle(Vec3{ x: x_position, y: y_position, z: 2.}, SQUARE_SIZE, SQUARE_SIZE, color);
    PointComponent { 0: sprite }
}

#[derive(Component)]
pub struct PointComponent(SpriteBundle);

#[derive(Component)]
pub struct RemainingPointsComponent;

#[derive(Component)]
pub struct PieceComponent;

#[derive(Component)]
pub struct NextPieceComponent;


pub fn init_board(mut commands: Commands) {
    commands.spawn_batch(generate_rectangle_with_border(Vec3{ x: 0., y: 0., z: 0.}, DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR));
    for x in 0..10 {
        for y in 0..20 {
            commands.spawn_batch(generate_cell_on_board(x, y));
        }
    }
    commands.spawn_batch(generate_rectangle_with_border(Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.}, DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR));

}

pub fn init_board_pieces(mut commands: Commands) {

}
