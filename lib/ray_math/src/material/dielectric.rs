use rand::Rng;

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

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
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
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..=1.0)
        {
            Vec3::reflect(&unit_direction, &hit.normal())
        } else {
            Vec3::refract(&unit_direction, &hit.normal(), refraction_ratio)
        };

        Some(ScatterResult {
            scattered: Ray::new(hit.point(), direction, ray_in.time()),
            attenuation: Color::one(),
        })
    }
}
