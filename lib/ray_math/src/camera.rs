use std::f64;

use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub struct CameraConfig {
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,
    pub vertical_field_of_view_degrees: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(cfg: CameraConfig) -> Self {
        let theta = cfg.vertical_field_of_view_degrees * (f64::consts::PI / 180.0);
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = cfg.aspect_ratio * viewport_height;

        let w = (cfg.look_from - cfg.look_at).normalized();
        let u = cfg.view_up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = cfg.look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
