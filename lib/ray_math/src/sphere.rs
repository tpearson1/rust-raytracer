use std::{f64, ops::Range, sync::Arc};

use crate::{material::Material, Aabb, HitResult, Hittable, Point3, Ray, Transform, Vec3};

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

    /// `point`: a given point on the sphere of radius one, centered at the origin.
    /// `u`: returned value [0,1] of angle around the Y axis from X=-1.
    /// `v`: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    ///     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_uv(point: &Point3) -> (f64, f64) {
        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + f64::consts::PI;
        (
            phi * 0.5 * f64::consts::FRAC_1_PI,
            theta * f64::consts::FRAC_1_PI,
        )
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
        let outward_normal = (point - center) / self.radius;
        Some(HitResult::new(
            ray,
            point,
            outward_normal,
            root,
            Self::get_uv(&outward_normal),
            Arc::clone(&self.material),
        ))
    }

    fn bounding_box(&self, time_range: Range<f64>) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius;

        let center0 = self.transform.position(time_range.start);
        let box0 = Aabb::new(center0 - offset, center0 + offset);

        let center1 = self.transform.position(time_range.end);
        let box1 = Aabb::new(center1 - offset, center1 + offset);

        Some(Aabb::surround(&box0, &box1))
    }
}
