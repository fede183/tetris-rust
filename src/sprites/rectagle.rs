use bevy::prelude::*;

pub struct Rectangle {
    height: f32, 
    wigth: f32, 
    fill_color: Color, 
}

impl Rectangle {
    pub fn new(height: f32, wigth: f32, fill_color: Color) -> Rectangle {
        Rectangle {
            height,
            wigth,
            fill_color,
        }
    }

    pub fn generate_sprite(&self, positions: Vec3) -> SpriteBundle {
        SpriteBundle {
           transform: Transform {
                translation: positions,
                scale: Vec3 {
                    x: self.wigth,
                    y: self.height,
                    z: positions.z,
                },
                ..default()
            },
            sprite: Sprite {
                color: self.fill_color,
                ..default()
            },
            ..default()
        }
    }
}

pub struct RectangleWithBorder {
    fill: Rectangle,
    border: Rectangle,
}

impl RectangleWithBorder {
    pub fn new(height: f32, wigth: f32, border_size: f32, fill_color: Color, border_color: Color) -> RectangleWithBorder {
        RectangleWithBorder {
            fill: Rectangle::new(height, wigth, fill_color),
            border: Rectangle::new(height + border_size, wigth + border_size, border_color),
        }
    }

    pub fn spawn(&self, commands: &mut Commands, positions: Vec3) {
        let border_sprite = self.border.generate_sprite(positions);
        let fill_sprite = self.fill.generate_sprite(positions);
        
        commands.spawn(border_sprite);
        commands.spawn(fill_sprite);
    }
}
