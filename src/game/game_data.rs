use std::usize;

use bevy::ecs::system::Resource;
use super::consts::{BOARD_HEIGHT, BOARD_WIGTH};
use super::piece::{self, Piece};
use super::point::Point;

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
        if !self.is_game_over() {
            self.check_complete_lines();
        }
    }

    pub fn descend(&mut self) -> bool {
        let it_descend = self.move_and_check(Piece::descend);

        if !it_descend {
            self.disable_piece();
        }

        it_descend
    }

    pub fn move_left(&mut self) -> bool {
        self.move_and_check(Piece::move_left)
    }

    pub fn move_right(&mut self) -> bool {
        self.move_and_check(Piece::move_right)
    }

    pub fn rotate(&mut self) -> bool {
        let rotate_piece_fn = |piece: &mut Piece| {
            piece.rotate();
        }; 
        self.move_and_check(rotate_piece_fn)
    }

    fn point_overlap_with_remain(&self, point: &Point) -> bool {
        for remain in &self.remaining_points {
            if point.equal(remain) {
                return true;
            }
        }

        false
    }

    fn is_valid_piece(&self) -> bool {
        let mut is_valid = true;
        for point in &self.piece.points {
            is_valid = is_valid && 0 <= point.x && point.x < BOARD_WIGTH;
            is_valid = is_valid && 0 <= point.y && point.y < BOARD_HEIGHT;
            is_valid = is_valid && !self.point_overlap_with_remain(point);
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

    fn disable_piece(&mut self) {
        let cloned_points = self.piece.points.clone();
        for point in cloned_points {
            self.remaining_points.push(point);
        }

        self.piece = self.next_piece.clone();
        self.next_piece = Piece::generate_random_piece();
    }

    pub fn is_game_over(&self) -> bool {
        for point in &self.remaining_points {
            if point.y == BOARD_HEIGHT {
                return true;
            }
        }

        false
    }

    fn check_complete_lines(&mut self) {
        let mut complete_lines_in_move = 0;
        for line in 0..BOARD_HEIGHT {
            self.check_complete_specific_line(line);
            complete_lines_in_move += 1;
        }

        self.score += (10 * complete_lines_in_move) + 2 * (complete_lines_in_move - 1);
    }

    fn check_complete_specific_line(&mut self, line: i32) {
        let points_in_line = self.remaining_points.iter().filter(|point| point.y == line);

        if points_in_line.count() == BOARD_WIGTH as usize {
            let remaining_points_minus_complete_line = self.remaining_points.iter_mut().filter(|point| point.y != line);
            let remaining_points_below = remaining_points_minus_complete_line.map(|point| {
                point.y += 1;
                *point
            });

            self.remaining_points = remaining_points_below.collect();
            self.lines += 1;
        }
    }
}
