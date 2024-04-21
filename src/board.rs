use bevy::prelude::*;
use crate::game::game_state::GameData;
use crate::{config::*, generate_rectangle};
use crate::common::generate_rectangle_with_border;
use crate::game::piece::Piece;
use crate::game::point::Point;
use crate::game::point::PointColor;

pub struct PieceComponentSprites {
    parent: SpatialBundle,
    children: Vec<SpriteBundle>,
}

#[derive(Component)]
pub struct PointComponent;

#[derive(Component)]
pub struct PieceComponent;

#[derive(Component)]
pub struct BoardPieceComponent;

#[derive(Component)]
pub struct NextPieceComponent;

#[derive(Component)]
pub struct RemainingPointsComponent;


fn get_color(color: PointColor) -> Color {

    match color {
        PointColor::RED => Color::RED,
        PointColor::BLUE => Color::BLUE,
        PointColor::YELLOW => Color::YELLOW,
        PointColor::GREEN => Color::GREEN,
    }
}

fn get_position_on_board(x: i32, y: i32) -> (f32, f32) {
    let x_position = SQUARE_SIZE* (x as f32);
    let y_position = SQUARE_SIZE* (y as f32);

    (-DISPLAY_FIRST_BOARD_POSITION_X + x_position, DISPLAY_FIRST_BOARD_POSITION_Y - y_position)
}

fn get_position_point_on_board(x: i32, y: i32) -> Vec3 {
    let (x_position, y_position) = get_position_on_board(x, y);

    Vec3 { x: x_position as f32, y: y_position as f32, z: 2.}
}

fn generate_cell_on_board(x: i32, y: i32) -> [SpriteBundle; 2] {
    let (x_position, y_position) = get_position_on_board(x, y);
    let square_size = SQUARE_SIZE - 5.;
    generate_rectangle_with_border(Vec3{ x: x_position, y: y_position, z: 1.}, square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR)
}

fn get_position_on_next_piece(x: i32, y: i32) -> (f32, f32) {
    let x_position = SQUARE_SIZE* (x as f32);
    let y_position = SQUARE_SIZE* (y as f32);

    (DISPLAY_NEXT_PIECE_POSITION_X + x_position, DISPLAY_NEXT_PIECE_POSITION_Y + y_position)
}

fn get_position_point_next_piece(x: i32, y: i32) -> Vec3 {
    let (x_position, y_position) = get_position_on_next_piece(x, y);

    Vec3 { x: x_position as f32, y: y_position as f32, z: 4.}
}

fn generate_point_in_next_piece(point: &Point) -> SpriteBundle {
    let color = get_color(point.color);
    let sprite = generate_rectangle(get_position_point_next_piece(point.x, point.y), SQUARE_SIZE, SQUARE_SIZE, color);

    sprite
}

fn generate_point_in_board(point: &Point) -> SpriteBundle {
    let color = get_color(point.color);
    let sprite = generate_rectangle(get_position_point_on_board(point.x, point.y), SQUARE_SIZE, SQUARE_SIZE, color);

    sprite
}

fn generate_next_piece(piece: &Piece) -> Vec<SpriteBundle> {
    let sprites = piece.points.iter().map(|point| generate_point_in_next_piece(point)).collect();

    sprites
}

fn generate_piece(piece: &Piece) -> Vec<SpriteBundle> {
    let sprites = piece.points.iter().map(|point| generate_point_in_board(point)).collect();

    sprites
}

fn get_next_piece_sprite(next_piece: &Piece) -> PieceComponentSprites {
    let parent = SpatialBundle::default();
    let children = generate_next_piece(next_piece);

    PieceComponentSprites {
        parent,
        children
    }
}

fn spawn_piece(mut commands: Commands, mut game_data: ResMut<GameData>) {
    let piece = &game_data.piece;
    let sprites_piece = generate_piece(piece);

    commands.spawn(BoardPieceComponent)
        .insert(PieceComponent)
        .with_children(|parent| { 
            for sprite in sprites_piece {
                parent.spawn(sprite);
            }
        });
}

fn spawn_next_piece(mut commands: Commands, mut game_data: &ResMut<GameData>) {
    let next_piece = &game_data.next_piece;
    let sprite_next_piece = get_next_piece_sprite(next_piece);

    println!("spawn the parent of next sprite with position: {}", sprite_next_piece.parent.transform.translation);
    commands.spawn(NextPieceComponent)
        .insert(PieceComponent)
        .insert(sprite_next_piece.parent)
        .with_children(|parent| { 
            for sprite in sprite_next_piece.children {
                println!("spawn the children of next sprite with position: {}", sprite.transform.translation);
                parent.spawn(sprite);
            }
        });
}

pub fn init_board(mut commands: Commands) {
    commands.spawn_batch(generate_rectangle_with_border(Vec3{ x: 0., y: 0., z: 0.}, DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR));
    for x in 0..10 {
        for y in 0..20 {
            commands.spawn_batch(generate_cell_on_board(x, y));
        }
    }
    commands.spawn_batch(generate_rectangle_with_border(Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.}, DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR));
}

pub fn init_board_pieces(mut commands: Commands, mut game_data: ResMut<GameData>) {
    // spawn_piece(commands, game_data);
    spawn_next_piece(commands, &game_data);
}
