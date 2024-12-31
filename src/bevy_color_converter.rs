use bevy::color::LinearRgba;

use crate::game::point::PointColor;

impl PointColor {
    pub fn get_color(&self) -> LinearRgba {
        match self {
            PointColor::RED => LinearRgba::RED,
            PointColor::BLUE => LinearRgba::BLUE,
            PointColor::YELLOW => LinearRgba::new(255., 255., 0., 255.),
            PointColor::GREEN => LinearRgba::GREEN,
        }
    }
}
