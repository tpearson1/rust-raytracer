use std::sync::Arc;

use crate::{texture::Texture, Ray, Vec3};

use super::{Material, ScatterResult};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> &dyn Texture {
        &*self.albedo
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        hit: &crate::HitResult,
        ray_in: &Ray,
    ) -> Option<ScatterResult> {
        let mut scatter_direction = hit.normal() + Vec3::random_unit(rng);
        // Catch degenerate scatter direction
        if scatter_direction.nearly_zero() {
            scatter_direction = hit.normal();
        }

        Some(ScatterResult {
            scattered: Ray::new(hit.point(), scatter_direction, ray_in.time()),
            attenuation: self.albedo.value(hit.uv(), &hit.point()),
        })
    }
}
