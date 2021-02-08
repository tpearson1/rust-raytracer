use std::sync::Arc;

use rand::Rng;
use ray_math::{
    material::{Dielectric, Lambertian, Metal},
    texture::{Checkered, SolidColor},
    BvhNode, CameraConfig, Color, Hittable, HittableList, LerpTransform, Point3, Sphere,
    StaticTransform, Vec3,
};

use super::SceneConfig;

pub fn scene(rng: &mut dyn rand::RngCore) -> SceneConfig {
    let mut world = HittableList::new();
    let time_range = 0.0..1.0;

    let texture = Arc::new(Checkered::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let ground_material = Arc::new(Lambertian::new(texture));
    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(0.0, -1000.0, 0.0)),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0..20);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..=1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let object: Arc<dyn Hittable> = match choose_mat {
                19 => {
                    // Glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    Arc::new(Sphere::from(StaticTransform::new(center), 0.2, mat))
                }
                16 | 17 | 18 => {
                    // Metal
                    let albedo = Color::random(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    Arc::new(Sphere::from(StaticTransform::new(center), 0.2, mat))
                }
                _ => {
                    // Diffuse
                    let albedo = Color::random_unit(rng) * Color::random_unit(rng);
                    let texture = Arc::new(SolidColor::new(albedo));
                    let mat = Arc::new(Lambertian::new(texture));

                    let transform = LerpTransform::new(
                        center,
                        center + Vec3::new(0.0, rng.gen_range(0.0..=0.5), 0.0),
                        time_range.clone(),
                    );
                    Arc::new(Sphere::from(transform, 0.2, mat))
                }
            };

            world.add(object);
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(0.0, 1.0, 0.0)),
        1.0,
        mat1,
    )));

    let texture = Arc::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let mat2 = Arc::new(Lambertian::new(texture));
    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(-4.0, 1.0, 0.0)),
        1.0,
        mat2,
    )));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::from(
        StaticTransform::new(Point3::new(4.0, 1.0, 0.0)),
        1.0,
        mat3,
    )));

    SceneConfig {
        root: BvhNode::new(rng, world, time_range),
        camera: CameraConfig {
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::zero(),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            vertical_field_of_view_degrees: 20.0,
            aspect_ratio: 3.0 / 2.0,
            aperture: 0.1,
            focus_distance: 10.0,
        },
    }
}
