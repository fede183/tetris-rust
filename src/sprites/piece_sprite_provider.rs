use bevy::prelude::*;
use crate::game::point::Point;
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
        let color = point.color.get_color();
        let position = self.mode.get_position(point.x, point.y).extend(3.);

        let rectangle = Rectangle::new(SQUARE_SIZE, SQUARE_SIZE, color);
        let sprite = rectangle.generate_sprite(position);

        sprite
    }

    pub fn generate_list_of_points(&self, points: &Vec<Point>) -> Vec<SpriteBundle> {
        let sprites = points.iter().map(|point| self.generate_point(&point)).collect();

        sprites
    }

    pub fn generate_piece(&self, piece: &Piece) -> Vec<SpriteBundle> {
        self.generate_list_of_points(&piece.points)
    }
}
