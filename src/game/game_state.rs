use bevy::ecs::schedule::States;
use bevy::ecs::system::Resource;

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

    pub fn descend(&mut self) {
        self.piece.descend();
    }

    pub fn move_left(&mut self) {
        self.piece.move_left();
    }

    pub fn move_right(&mut self) {
        self.piece.move_left();
    }

    pub fn rotate(&mut self) {
        self.piece.rotate();
    }
}
