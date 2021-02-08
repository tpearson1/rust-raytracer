use crate::{Color, Point3};

use super::Texture;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: (f64, f64), _pointt: &Point3) -> Color {
        self.color
    }
}
