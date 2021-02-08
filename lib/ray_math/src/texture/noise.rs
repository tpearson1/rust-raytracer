use rand::Rng;

use crate::{Color, Point3};

use super::Texture;

pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn new(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            perlin: Perlin::new(rng),
        }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: (f64, f64), point: &Point3) -> Color {
        Color::one() * self.perlin.noise(point)
    }
}

struct Perlin {
    random: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

const POINT_COUNT: i32 = 256;

impl Perlin {
    fn new(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            random: (0..POINT_COUNT).map(|_| rng.gen_range(0.0..=1.0)).collect(),
            perm_x: Self::perlin_generate_perm(rng),
            perm_y: Self::perlin_generate_perm(rng),
            perm_z: Self::perlin_generate_perm(rng),
        }
    }

    fn noise(&self, point: &Point3) -> f64 {
        let i = (4.0 * point.x()) as i32 & 255;
        let j = (4.0 * point.y()) as i32 & 255;
        let k = (4.0 * point.z()) as i32 & 255;
        self.random
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }

    fn perlin_generate_perm(rng: &mut dyn rand::RngCore) -> Vec<i32> {
        let mut p: Vec<_> = (0..POINT_COUNT).collect();
        Self::permute(rng, &mut p, POINT_COUNT as usize);
        p
    }

    fn permute(rng: &mut dyn rand::RngCore, p: &mut Vec<i32>, n: usize) {
        for i in (1..n).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }
}
