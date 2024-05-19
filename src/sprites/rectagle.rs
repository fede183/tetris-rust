use bevy::prelude::*;

pub fn generate_rectangle(positions: Vec3, height: f32, wigth: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: positions,
            scale: Vec3 {
                x: wigth,
                y: height,
                z: positions.z,
            },
            ..default()
        },
        sprite: Sprite {
            color,
            ..default()
        },
        ..default()
    }
}

pub struct RectangleWithBorder {
    height: f32, 
    wigth: f32, 
    border_size: f32, 
    fill_color: Color, 
    border_color: Color
}

impl RectangleWithBorder {
    pub fn new(height: f32, wigth: f32, border_size: f32, fill_color: Color, border_color: Color) -> RectangleWithBorder {
        RectangleWithBorder {
            height: height,
            wigth: wigth,
            border_size: border_size,
            fill_color: fill_color,
            border_color: border_color,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, positions: Vec3) {
        let border = generate_rectangle(Vec3 { z: positions.z + 1., ..positions }, self.height, self.wigth, self.fill_color);
        let fill = generate_rectangle(positions, self.height + self.border_size, self.wigth + self.border_size, self.border_color);
        
        commands.spawn(fill);
        commands.spawn(border);
    }
}
