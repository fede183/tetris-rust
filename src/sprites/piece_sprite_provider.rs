use bevy::prelude::*;
use crate::game::point::Point;
use crate::game::piece::Piece;
use crate::config::SQUARE_SIZE;
use crate::sprites::point_mode::PointMode;
use crate::sprites::rectagle::generate_rectangle;


pub struct PieceSpriteProvider {
    pub mode: PointMode,
}

impl PieceSpriteProvider {

    pub fn new(mode: &PointMode) -> PieceSpriteProvider {
        PieceSpriteProvider { mode: mode.clone() }
    }

    fn generate_point(&self, point: &Point) -> SpriteBundle {
        let color = point.color.get_color();
        let x_position = (point.x as f32) * SQUARE_SIZE;
        let y_position = (point.y as f32) * SQUARE_SIZE;
        let position = Vec3 { x: x_position, y: y_position, z: 3. };
        let sprite = generate_rectangle(position, SQUARE_SIZE, SQUARE_SIZE, color);

        sprite
    }

    pub fn generate_piece(&self, piece: &Piece) -> Vec<SpriteBundle> {
        let sprites = piece.points.iter().map(|point| self.generate_point(point)).collect();

        sprites
    }
}
