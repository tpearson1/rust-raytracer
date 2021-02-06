use crate::{Color, Ray, Vec3};

use super::{Material, ScatterResult};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        _rng: &mut dyn rand::RngCore,
        hit: &crate::HitResult,
        ray_in: &Ray,
    ) -> Option<ScatterResult> {
        let refraction_ratio = if hit.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = (-unit_direction).dot(&hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(&unit_direction, &hit.normal())
        } else {
            Vec3::refract(&unit_direction, &hit.normal(), refraction_ratio)
        };

        Some(ScatterResult {
            scattered: Ray::new(hit.point(), direction),
            attenuation: Color::one(),
        })
    }
}
