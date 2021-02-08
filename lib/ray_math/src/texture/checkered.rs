use std::sync::Arc;

use crate::{Color, Point3};

use super::Texture;

pub struct Checkered {
    scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checkered {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            scale: 10.0,
            even,
            odd,
        }
    }
}

impl Texture for Checkered {
    fn value(&self, uv: (f64, f64), point: &Point3) -> Color {
        let sines = (self.scale * point.x()).sin()
            * (self.scale * point.y()).sin()
            * (self.scale * point.z()).sin();
        if sines < 0.0 {
            self.odd.value(uv, point)
        } else {
            self.even.value(uv, point)
        }
    }
}
