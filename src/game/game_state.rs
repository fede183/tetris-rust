use bevy::ecs::schedule::States;
use bevy::ecs::system::Resource;

use super::consts::{BOARD_HEIGHT, BOARD_WIGTH};
use super::piece::Piece;
use super::point::Point;

#[derive(States, Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct GameData {
    pub piece: Piece,
    pub next_piece: Piece,
    pub remaining_points: Vec<Point>,
    pub score: i32,
    pub lines: i32,
}

impl GameData {
    pub fn new_game() -> GameData {
        let piece = Piece::generate_random_piece();
        let next_piece = Piece::generate_random_piece();
        let remaining_points = Vec::new();
        let score = 0;
        let lines = 0;

        GameData { piece, next_piece, remaining_points, score, lines }
    }

    pub fn cycle(&mut self) {
        self.descend();
    }

    pub fn descend(&mut self) -> bool {
        self.move_and_check(Piece::descend)
    }

    pub fn move_left(&mut self) -> bool {
        self.move_and_check(Piece::move_left)
    }

    pub fn move_right(&mut self) -> bool {
        self.move_and_check(Piece::move_right)
    }

    pub fn rotate(&mut self) -> bool {
        self.move_and_check(Piece::rotate)
    }

    fn is_valid_piece(&self) -> bool {
        let mut is_valid = true;
        for point in &self.piece.points {
            is_valid = is_valid && 0 <= point.x && point.x < BOARD_WIGTH;
            is_valid = is_valid && 0 <= point.y && point.y < BOARD_HEIGHT;
        }
        is_valid
    }

    fn move_and_check(&mut self, movement_function: fn(&mut Piece)) -> bool {
        let old_piece = self.piece.clone();
        movement_function(&mut self.piece);
        if !self.is_valid_piece() {
            self.piece = old_piece;
            return false;
        }

        true
    }
}
