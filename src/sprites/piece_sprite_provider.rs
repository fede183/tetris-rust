use bevy::prelude::*;
use crate::game::point::{Point, PointColor};
use crate::game::piece::Piece;
use crate::config::SQUARE_SIZE;
use crate::sprites::point_mode::PointMode;
use crate::sprites::rectagle::Rectangle;


pub struct PieceSpriteProvider {
    pub mode: PointMode,
}

impl PieceSpriteProvider {

    pub fn new(mode: &PointMode) -> PieceSpriteProvider {
        PieceSpriteProvider { mode: mode.clone() }
    }

    pub fn generate_point(&self, point: &Point) -> SpriteBundle {

        let position = self.generate_position(point);

        self.generate_sprite_using_coordinates(position, point.color)
    }

    fn generate_position(&self, point: &Point) -> Vec3 {
        let x_position = (point.x as f32) * SQUARE_SIZE;
        let y_position = (point.y as f32) * SQUARE_SIZE;

        let position = Vec3 { x: x_position, y: y_position, z: 3. };

        position
    }

    fn generate_sprite_using_coordinates(&self, position: Vec3, color: PointColor) -> SpriteBundle {
        let color = color.get_color();

        let rectangle = Rectangle::new(SQUARE_SIZE, SQUARE_SIZE, color);
        let sprite = rectangle.generate_sprite(position);

        sprite
    }

    pub fn generate_piece(&self, piece: &Piece) -> Vec<SpriteBundle> {
        let (x, y) = piece.center_point;

        let sprites = piece.points.iter().map(|point| {
            let mut positions = self.generate_position(point);

            positions.x -= x * SQUARE_SIZE;
            positions.y -= y * SQUARE_SIZE;

            (positions, point.color)
        }).map(|data| self.generate_sprite_using_coordinates(data.0, data.1)).collect();

        sprites
    }
}
