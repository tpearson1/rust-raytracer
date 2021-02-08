use rand::Rng;

use crate::{Color, Point3};

use super::Texture;

pub struct Noise {
    scale: f64,
    perlin: Perlin,
}

impl Noise {
    pub fn new(rng: &mut dyn rand::RngCore, scale: f64) -> Self {
        Self {
            scale,
            perlin: Perlin::new(rng),
        }
    }
}

impl Texture for Noise {
    fn value(&self, _uv: (f64, f64), point: &Point3) -> Color {
        Color::one() * self.perlin.noise(&(self.scale * *point))
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
        let mut u = point.x() - point.x().floor();
        let mut v = point.y() - point.y().floor();
        let mut w = point.z() - point.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    c[di as usize][dj as usize][dk as usize] = self.random[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        (0..2)
            .flat_map(move |i| {
                (0..2).flat_map(move |j| {
                    (0..2).map(move |k| {
                        let fi = i as f64;
                        let fj = j as f64;
                        let fk = k as f64;
                        (fi * u + (1.0 - fi) * (1.0 - u))
                            * (fj * v + (1.0 - fj) * (1.0 - v))
                            * (fk * w + (1.0 - fk) * (1.0 - w))
                            * c[i][j][k]
                    })
                })
            })
            .sum()
    }
}
