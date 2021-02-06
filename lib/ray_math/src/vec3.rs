use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{io::Write, ops::Neg};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
        a * (1.0 - t) + b * t
    }

    pub fn write_color<TWrite: Write>(&self, writer: &mut TWrite) -> std::io::Result<()> {
        let ir = (255.999 * clamp(self.r(), 0.0, 0.999)) as usize;
        let ig = (255.999 * clamp(self.g(), 0.0, 0.999)) as usize;
        let ib = (255.999 * clamp(self.b(), 0.0, 0.999)) as usize;
        write!(writer, "{} {} {}\n", ir, ig, ib)
    }

    pub fn random(rng: &mut dyn rand::RngCore, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut dyn rand::RngCore) -> Vec3 {
        // Pick a point in the unit cube, and reject until the point is in the
        // unit sphere
        loop {
            let point = Vec3::random(rng, -1.0, 1.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_unit(rng: &mut dyn rand::RngCore) -> Vec3 {
        Self::random_in_unit_sphere(rng).normalized()
    }

    pub fn random_in_hemisphere(rng: &mut dyn rand::RngCore, normal: &Vec3) -> Vec3 {
        let result = Self::random_in_unit_sphere(rng);
        if result.dot(normal) > 0.0 {
            result
        } else {
            -result
        }
    }

    pub fn nearly_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
        *vec - 2.0 * vec.dot(normal) * *normal
    }

    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv).dot(normal).min(1.0);
        let ray_out_perp = etai_over_etat * (*uv + cos_theta * *normal);
        let ray_out_parallel = -((1.0 - ray_out_perp.length_squared()).abs()).sqrt() * *normal;
        ray_out_perp + ray_out_parallel
    }
}

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        return min;
    }
    if v > max {
        return max;
    }
    return v;
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
