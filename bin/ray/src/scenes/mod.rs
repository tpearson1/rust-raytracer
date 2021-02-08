use ray_math::{BvhNode, CameraConfig};

mod random;
mod two_perlin_spheres;
mod two_spheres;

pub enum SceneOption {
    Random,
    TwoSpheres,
    TwoPerlinSpheres,
}

pub struct SceneConfig {
    pub root: BvhNode,
    pub camera: CameraConfig,
}

pub fn make_scene(rng: &mut dyn rand::RngCore, scene: SceneOption) -> SceneConfig {
    match scene {
        SceneOption::Random => random::scene(rng),
        SceneOption::TwoSpheres => two_spheres::scene(rng),
        SceneOption::TwoPerlinSpheres => two_perlin_spheres::scene(rng),
    }
}
