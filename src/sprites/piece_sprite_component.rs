use bevy::prelude::*;
use crate::game::piece::Piece;
use crate::sprites::point_mode::PointMode;

use super::piece_sprite_provider::PieceSpriteProvider;
use super::rectagle::RectangleBundle;

pub struct PieceComponentSprites {
    pub parent: (Transform, Visibility),
    pub children: Vec<RectangleBundle>,
}

impl PieceComponentSprites {
    pub fn new(piece: &Piece, point_mode: &PointMode) -> PieceComponentSprites {
        let translation = point_mode.get_initial_piece_position(piece);
        let parent = (Transform::from_xyz(translation.x, translation.y, translation.z), Visibility::Visible);

        let provider = PieceSpriteProvider::new();
        let children = provider.generate_piece(piece);

        PieceComponentSprites {
            parent,
            children
        }
    }

    pub fn spawn_component(component: PieceComponentSprites, commands: &mut Commands, piece_entity: Entity) {
        let parent_entity = commands.entity(piece_entity).insert(component.parent).id();

        for child in component.children {
            let child_entity = commands.spawn(child).id();
            commands.entity(parent_entity).add_child(child_entity);
        }
    }
}
