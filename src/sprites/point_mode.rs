use crate::config::*;
use bevy::prelude::*;

#[derive(Clone)]
pub enum PointMode {
    Board,
    Next,
}

impl PointMode {
    pub fn get_initial_position(&self) -> Vec2 {
        match self {
            PointMode::Board => Vec2{ x: DISPLAY_FIRST_BOARD_POSITION_X, y: DISPLAY_FIRST_BOARD_POSITION_Y },
            PointMode::Next => Vec2{ x: DISPLAY_FIRST_NEXT_PIECE_POSITION_X, y: DISPLAY_FIRST_NEXT_PIECE_POSITION_Y },
        }
    }

    pub fn get_position(&self, x: i32, y: i32) -> Vec2 {
        let x_position = SQUARE_SIZE* (x as f32);
        let y_position = SQUARE_SIZE* (y as f32);

        let init_position = self.get_initial_position();

        let (x_position, y_position) = match self {
            PointMode::Board => (init_position.x + x_position, init_position.y - y_position),
            PointMode::Next => (init_position.x + x_position, init_position.y + y_position),
        };

        Vec2 { x: x_position, y: y_position }
    }

    pub fn get_initial_piece_position(&self) -> Vec3 {
        match self {
            PointMode::Board => self.get_position(0, 1).extend(3.),
            PointMode::Next => self.get_position(0, 0).extend(3.),
        }
    }
}