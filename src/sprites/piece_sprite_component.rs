use bevy::prelude::*;
use crate::game::piece::Piece;
use crate::sprites::point_mode::PointMode;

use super::piece_sprite_provider::PieceSpriteProvider;

pub struct PieceComponentSprites {
    pub parent: SpatialBundle,
    pub children: Vec<SpriteBundle>,
}

impl PieceComponentSprites {
    pub fn new(piece: &Piece, point_mode: &PointMode) -> PieceComponentSprites {
        let translation = point_mode.get_initial_piece_position();
        let parent = SpatialBundle {
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        };

        let provider = PieceSpriteProvider::new(point_mode);
        let children = provider.generate_piece(piece);

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
