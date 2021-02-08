use std::sync::Arc;

use ray_math::{
    material::Lambertian,
    texture::{Checkered, SolidColor},
    BvhNode, CameraConfig, Color, HittableList, Point3, Sphere, StaticTransform, Vec3,
};

use super::SceneConfig;

pub fn scene(rng: &mut dyn rand::RngCore) -> SceneConfig {
    let mut world = HittableList::new();
    let time_range = 0.0..1.0;

    let texture = Arc::new(Checkered::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(0.0, -10.0, 0.0)),
        10.0,
        Arc::new(Lambertian::new(texture.clone())),
    )));

    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(0.0, 10.0, 0.0)),
        10.0,
        Arc::new(Lambertian::new(texture)),
    )));

    SceneConfig {
        root: BvhNode::new(rng, world, time_range),
        camera: CameraConfig {
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::zero(),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            vertical_field_of_view_degrees: 20.0,
            aspect_ratio: 3.0 / 2.0,
            aperture: 0.0,
            focus_distance: 10.0,
        },
    }
}
