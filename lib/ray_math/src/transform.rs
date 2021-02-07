use std::ops::Range;

use crate::Point3;

pub trait Transform: Send + Sync {
    fn position(&self, time: f64) -> Point3;
}

pub struct StaticTransform {
    position: Point3,
}

impl StaticTransform {
    pub fn new(position: Point3) -> Self {
        Self { position }
    }
}

impl Transform for StaticTransform {
    fn position(&self, _time: f64) -> Point3 {
        self.position
    }
}

pub struct LerpTransform {
    from: Point3,
    to: Point3,
    time_range: Range<f64>,
}

impl LerpTransform {
    pub fn new(from: Point3, to: Point3, time_range: Range<f64>) -> Self {
        Self {
            from,
            to,
            time_range,
        }
    }
}

impl Transform for LerpTransform {
    fn position(&self, time: f64) -> Point3 {
        let duration = self.time_range.end - self.time_range.start;
        let elapsed = time - self.time_range.start;
        self.from + (elapsed / duration) * (self.to - self.from)
    }
}
