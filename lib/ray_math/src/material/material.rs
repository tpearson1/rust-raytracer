use crate::{Color, HitResult, Ray};

pub trait Material {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        hit: &HitResult,
        ray_in: &Ray,
    ) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}
