use std::sync::Arc;

use crate::{material::Material, HitResult, Hittable, Ray, Transform};

pub struct Sphere<T> {
    transform: T,
    radius: f64,
    material: Arc<dyn Material>,
}

impl<T: Transform> Sphere<T> {
    pub fn from(transform: T, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            transform,
            radius,
            material,
        }
    }

    pub fn transform(&self) -> &T {
        &self.transform
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl<T: Transform> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let center = self.transform().position(ray.time());
        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitResult::new(
            ray,
            point,
            (point - center) / self.radius,
            root,
            Arc::clone(&self.material),
        ))
    }
}
