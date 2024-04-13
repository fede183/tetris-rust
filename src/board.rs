use bevy::prelude::*;
use crate::{config::*, draw_cell_on_board, draw_piece_on_board};
use crate::common::{draw_next_piece, draw_rectangle_with_border};
use rand::Rng;

const COLORS: [Color; 5] = [Color::RED, Color::BLUE, Color::YELLOW, Color::GREEN, Color::PURPLE];

fn generate_new_piece() -> Piece {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..=4);
    let color_int: usize = rng.gen_range(0..=4);
    let figures = [
        [(0, 0), (0, 1), (0, 2), (1, 2)],
        [(1, 0), (1, 1), (1, 2), (0, 2)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (2, 0)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (1, 0), (2, 0), (3, 1)],
    ];
    let color: Color = *(COLORS.get(color_int).unwrap());

    let mut points: Vec<PointOnBoard> = Vec::new();
    for index_in_figure in 0..4 {
        let (x, y) = figures[index][index_in_figure];
        points.push(PointOnBoard { x, y, color });
    }

    Piece { 0: points }
}

#[derive(Component)]
pub struct PointOnBoard {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

#[derive(Component)]
pub struct RemainingPointsOnBoard(Vec<PointOnBoard>);

#[derive(Component)]
pub struct Piece(pub Vec<PointOnBoard>);

#[derive(Component)]
pub struct NextPiece(pub Piece);

impl Piece {
    pub fn descend(&mut self) {
        for point in &mut self.0 {
            point.y += 1;
        }
    }
}


pub fn init_board(mut commands: Commands) {
    commands.spawn((generate_new_piece(), RemainingPointsOnBoard { 0: Vec::new() }));
    commands.spawn(NextPiece { 0: generate_new_piece() });
}

pub fn setup_board(mut commands: Commands, query: Query<(&Piece, &RemainingPointsOnBoard)>) {
    draw_rectangle_with_border(&mut commands, Vec3{ x: 0., y: 0., z: 0.}, DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);
    for x in 0..10 {
        for y in 0..20 {
            draw_cell_on_board(&mut commands, x, y);
        }
    }

    let (piece, remaining) = query.single(); 
    draw_piece_on_board(&mut commands, piece); 
    
}

pub fn setup_next_piece_board(mut commands: Commands, query: Query<&NextPiece>) {
    draw_rectangle_with_border(&mut commands, Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.}, DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR);

    let next_piece = query.single();
    draw_next_piece(&mut commands, &next_piece.0);
}
