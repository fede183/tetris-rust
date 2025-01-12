use bevy::prelude::*;
use crate::game::consts::{BOARD_HEIGHT, BOARD_WIGTH};
use crate::game::game_data::GameData;
use crate::config::*;
use crate::sprites::rectagle::RectangleWithBorder;
use crate::sprites::point_mode::PointMode;
use crate::sprites::piece_sprite_component::PieceComponentSprites;
use crate::sprites::remaining_points_sprite_component::RemainingPointsComponentSprites;

#[derive(Component)]
pub struct BoardPieceComponent;

#[derive(Component)]
pub struct NextPieceComponent;

#[derive(Component)]
pub struct RemainingPointsComponent;

fn generate_cell_on_board(commands: &mut Commands) {
    let square_size = SQUARE_SIZE - 5.;
    let cell = RectangleWithBorder::new(square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR);

    for x in 0..BOARD_WIGTH {
        for y in 0..BOARD_HEIGHT {
            cell.spawn(commands, PointMode::Board.get_position(x as f32, y as f32).extend(2.));
        }
    }
}

pub fn init_board(mut commands: Commands) {
    RectangleWithBorder::new(DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR).spawn(&mut commands, Vec3{ x: 0., y: 0., z: 0.});
    
    generate_cell_on_board(&mut commands);

    RectangleWithBorder::new(DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR).spawn(&mut commands, Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.});
}

pub fn spawn_piece(commands: &mut Commands, game_data: &ResMut<GameData>) {
    let piece = &game_data.piece;
    let sprite_piece = PieceComponentSprites::new(piece, &PointMode::Board);

    let piece_entity = commands.spawn(BoardPieceComponent).id();

    PieceComponentSprites::spawn_component(sprite_piece, commands, piece_entity);
}

pub fn spawn_next_piece(commands: &mut Commands, game_data: &ResMut<GameData>) {
    let next_piece = &game_data.next_piece;
    let sprite_next_piece = PieceComponentSprites::new(next_piece, &PointMode::Next);

    let piece_entity = commands.spawn(NextPieceComponent).id();

    PieceComponentSprites::spawn_component(sprite_next_piece, commands, piece_entity);
}

pub fn spawn_remaining_points(commands: &mut Commands, game_data: &ResMut<GameData>) {
    let remaining_points = &game_data.remaining_points;
    let mut sprite_remaining = RemainingPointsComponentSprites::new();
    sprite_remaining.add_points(remaining_points);

    let remaining_entity = commands.spawn(RemainingPointsComponent).id();

    RemainingPointsComponentSprites::spawn_component(sprite_remaining, commands, remaining_entity);
}

pub fn init_board_pieces(mut commands: Commands, game_data: ResMut<GameData>) {
    spawn_piece(&mut commands, &game_data);
    spawn_next_piece(&mut commands, &game_data);
    spawn_remaining_points(&mut commands, &game_data);
}
