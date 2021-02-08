use crate::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, uv: (f64, f64), point: &Point3) -> Color;
}
