use bevy::prelude::*;
use crate::{config::*, generate_rectangle};
use crate::game::piece::Piece;
use crate::game::point::Point;

pub struct PieceComponentSprites {
    pub parent: SpatialBundle,
    pub children: Vec<SpriteBundle>,
}

impl PieceComponentSprites {
    pub fn new(children: Vec<SpriteBundle>) -> PieceComponentSprites {
        let parent = SpatialBundle::default();

        PieceComponentSprites {
            parent,
            children
        }
    }

    pub fn spawn_piece_component(component: PieceComponentSprites, commands: &mut Commands, piece_entity: Entity) {
        let parent_entity = commands.entity(piece_entity).insert(component.parent).id();

        for child in component.children {
            let child_entity = commands.spawn(child).id();
            commands.entity(parent_entity).push_children(&[child_entity]);
        }
    }
}

pub enum PointMode {
    Board,
    Next,
}

impl PointMode {
    pub fn get_position(&self, x: i32, y: i32, z: i32) -> Vec3 {
        let x_position = SQUARE_SIZE* (x as f32);
        let y_position = SQUARE_SIZE* (y as f32);

        let (x_position, y_position) = match self {
            PointMode::Board => (-DISPLAY_FIRST_BOARD_POSITION_X + x_position, DISPLAY_FIRST_BOARD_POSITION_Y - y_position),
            PointMode::Next => (DISPLAY_FIRST_NEXT_PIECE_POSITION_X + x_position, DISPLAY_FIRST_NEXT_PIECE_POSITION_Y + y_position),
        };

        Vec3 { x: x_position, y: y_position, z: z as f32}
    }
}

pub struct PieceToSpriteProvider {
    pub mode: PointMode,
}

impl PieceToSpriteProvider {

    fn generate_point(&self, point: &Point) -> SpriteBundle {
        let color = point.color.get_color();
        let mode = &self.mode;
        let position = mode.get_position(point.x, point.y, 3);
        let sprite = generate_rectangle(position, SQUARE_SIZE, SQUARE_SIZE, color);

        sprite
    }

    pub fn generate_piece(&self, piece: &Piece) -> PieceComponentSprites {
        let sprites = piece.points.iter().map(|point| self.generate_point(point)).collect();

        PieceComponentSprites::new(sprites)
    }
}
