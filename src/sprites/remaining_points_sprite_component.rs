use bevy::prelude::*;
use crate::game::point::Point;
use crate::sprites::point_mode::PointMode;
use super::piece_sprite_provider::PieceSpriteProvider;
use super::rectagle::RectangleBundle;

pub struct RemainingPointsComponentSprites {
    pub parent: (Transform, Visibility),
    pub children: Vec<RectangleBundle>,
}

impl RemainingPointsComponentSprites {
    pub fn new() -> RemainingPointsComponentSprites {
        let point_mode = PointMode::Board;
        let translation = point_mode.get_position(0.0, 0.0).extend(3.);
        let parent = (Transform::from_xyz(translation.x, translation.y, translation.z), Visibility::Visible);

        let children = Vec::new();

        RemainingPointsComponentSprites {
            parent,
            children
        }
    }

    pub fn spawn_component(component: RemainingPointsComponentSprites, commands: &mut Commands, remaining_entity: Entity) {
        let parent_entity = commands.entity(remaining_entity).insert(component.parent).id();

        for child in component.children {
            let child_entity = commands.spawn(child).id();
            commands.entity(parent_entity).add_child(child_entity);
        }
    }

    fn add_point(&mut self, point: &Point) {
        let provider = PieceSpriteProvider::new();
        let point_sprite = provider.generate_point(point);
        self.children.push(point_sprite);
    }

    pub fn add_points(&mut self, points: &Vec<Point>) {
        for point in points {
            self.add_point(point);
        }
    }
}
