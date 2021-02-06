use crate::{Color, Ray, Vec3};

use super::{Material, ScatterResult};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        hit: &crate::HitResult,
        _ray_in: &Ray,
    ) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal() + Vec3::random_unit(rng);
        // Catch degenerate scatter direction
        if scatter_direction.nearly_zero() {
            scatter_direction = hit.normal();
        }

        Some(ScatterResult {
            scattered: Ray::new(hit.point(), scatter_direction),
            attenuation: self.albedo,
        })
    }
}
