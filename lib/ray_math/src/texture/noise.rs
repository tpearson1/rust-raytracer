use rand::Rng;

use crate::{Color, Point3, Vec3};

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
        Color::one() * 0.5 * (1.0 + self.perlin.noise(&(self.scale * *point)))
    }
}

struct Perlin {
    random: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

const POINT_COUNT: i32 = 256;

impl Perlin {
    fn new(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            random: (0..POINT_COUNT)
                .map(|_| Vec3::random(rng, -1.0, 1.0).normalized())
                .collect(),
            perm_x: Self::perlin_generate_perm(rng),
            perm_y: Self::perlin_generate_perm(rng),
            perm_z: Self::perlin_generate_perm(rng),
        }
    }

    fn noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let q = self.perm_x[((i + di as i32) & 255) as usize];
                    let r = self.perm_y[((j + dj as i32) & 255) as usize];
                    let s = self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.random[(q ^ r ^ s) as usize];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
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

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        (0..2)
            .flat_map(move |i| {
                (0..2).flat_map(move |j| {
                    (0..2).map(move |k| {
                        let fi = i as f64;
                        let fj = j as f64;
                        let fk = k as f64;
                        let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));
                        (fi * uu + (1.0 - fi) * (1.0 - uu))
                            * (fj * vv + (1.0 - fj) * (1.0 - vv))
                            * (fk * ww + (1.0 - fk) * (1.0 - ww))
                            * c[i][j][k].dot(&weight_v)
                    })
                })
            })
            .sum()
    }
}
