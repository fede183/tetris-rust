use bevy::prelude::*;
use crate::game::point::Point;
use crate::game::piece::Piece;
use crate::config::SQUARE_SIZE;
use crate::sprites::point_mode::PointMode;
use crate::sprites::rectagle::generate_rectangle;
use crate::sprites::piece_sprite_component::PieceComponentSprites;

pub struct PieceToSpriteProvider {
    pub mode: PointMode,
}

impl PieceToSpriteProvider {

    fn generate_point(&self, point: &Point) -> SpriteBundle {
        let color = point.color.get_color();
        let x_position = SQUARE_SIZE* (point.x as f32);
        let y_position = SQUARE_SIZE* (point.y as f32);
        let position = Vec3 { x: x_position, y: y_position, z: 3. };
        let sprite = generate_rectangle(position, SQUARE_SIZE, SQUARE_SIZE, color);

        sprite
    }

    pub fn generate_piece(&self, piece: &Piece) -> PieceComponentSprites {
        let sprites = piece.points.iter().map(|point| self.generate_point(point)).collect();

        PieceComponentSprites::new(sprites, &self.mode)
    }
}
