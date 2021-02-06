use std::sync::Arc;

use crate::{material::Material, Point3, Ray, Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}

pub struct HitResult {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Arc<dyn Material>,
}

impl HitResult {
    pub fn new(
        ray: &Ray,
        point: Point3,
        outward_normal: Vec3,
        t: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}
