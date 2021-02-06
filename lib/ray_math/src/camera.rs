use std::f64;

use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

pub struct CameraConfig {
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,
    pub vertical_field_of_view_degrees: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_distance: f64,
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
        let horizontal = cfg.focus_distance * viewport_width * u;
        let vertical = cfg.focus_distance * viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - cfg.focus_distance * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,

            u,
            v,
            lens_radius: cfg.aperture * 0.5,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }

    pub fn get_ray_defocused(&self, rng: &mut dyn rand::RngCore, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
