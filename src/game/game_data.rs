use std::usize;

use bevy::ecs::system::Resource;
use super::consts::{BOARD_HEIGHT, BOARD_WIGTH};
use super::piece::Piece;
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
            self.descend();
        }
    }

    pub fn descend(&mut self) -> bool {
        let it_descend = self.move_and_check(Piece::descend);

        if !it_descend {
            self.disable_piece();
            self.check_complete_lines();
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
        let old_piece = self.piece.clone();
        rotate_piece_fn(&mut self.piece);

        if !self.is_valid_piece_remain() {
            self.piece = old_piece;
            return false;
        }

        while !self.is_valid_piece_left_border() && self.move_right() {}
        while !self.is_valid_piece_right_border() && self.move_left() {}

        true
    }

    fn point_overlap_with_remain(&self, point: &Point) -> bool {
        for remain in &self.remaining_points {
            if point.equal(remain) {
                return true;
            }
        }

        false
    }

    fn is_valid_piece_left_border(&self) -> bool {
        let mut is_valid = true;
        for point in &self.piece.points {
            is_valid = is_valid && 0 <= point.x && point.x < BOARD_WIGTH;
        }
        is_valid
    }
        
    fn is_valid_piece_right_border(&self) -> bool {
        let mut is_valid = true;
        for point in &self.piece.points {
            is_valid = is_valid && 0 <= point.y && point.y < BOARD_HEIGHT;
        }
        is_valid
    }
        
    fn is_valid_piece_remain(&self) -> bool {
        let mut is_valid = true;
        for point in &self.piece.points {
            is_valid = is_valid && !self.point_overlap_with_remain(point);
        }
        is_valid
    }

    fn is_valid_piece(&self) -> bool {
        let mut is_valid = true;
        is_valid = is_valid && self.is_valid_piece_left_border();
        is_valid = is_valid && self.is_valid_piece_right_border();
        is_valid = is_valid && self.is_valid_piece_remain();

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
            if point.y == BOARD_HEIGHT - 1 && !self.is_valid_piece() {
                return true;
            }
        }

        false
    }

    fn check_complete_lines(&mut self) {
        for line in 0..BOARD_HEIGHT {
            self.check_complete_specific_line(line);
        }

        if self.lines > 0 {
            self.score += (10 * self.lines) + 2 * (self.lines - 1);
        }
    }

    fn check_complete_specific_line(&mut self, line: i32) {
        let points_in_line = self.remaining_points.iter().filter(|point| point.y == line);

        if points_in_line.count() == BOARD_WIGTH as usize {
            let remaining_points_minus_complete_line = self.remaining_points.iter_mut().filter(|point| point.y != line);
            let remaining_points_below = remaining_points_minus_complete_line.map(|point| {
                if point.y <= line {
                    point.y += 1;
                }
                *point
            });

            self.remaining_points = remaining_points_below.collect();
            self.lines += 1;
        }
    }
}
