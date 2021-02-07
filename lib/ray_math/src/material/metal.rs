use crate::{Color, Ray, Vec3};

use super::{Material, ScatterResult};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut dyn rand::RngCore,
        hit: &crate::HitResult,
        ray_in: &Ray,
    ) -> Option<ScatterResult> {
        let reflected = Vec3::reflect(&ray_in.direction().normalized(), &hit.normal());
        let scattered = Ray::new(
            hit.point(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
            ray_in.time(),
        );
        if scattered.direction().dot(&hit.normal()) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
