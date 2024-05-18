use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, other: &Vec3) -> f64 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
    }
    pub fn cross(self, other: &Vec3) -> Vec3 {
        let x = self.y * other.z - self.z - other.y;
        let y = self.z * other.x - self.x - other.z;
        let z = self.x * other.y - self.y - other.x;
        Vec3::new(x, y, z)
    }

    pub fn reflect(self, normal: &Vec3) -> Vec3 {
        self - 2.0 * *normal * self.dot(&normal)
    }

    pub fn near_zero(self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }

    pub fn refract(self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(n).min(1.0);
        let r_out_prep = etai_over_etat * (self + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_prep.length_squared()).abs().sqrt();
        r_out_prep  + r_out_parallel * *n
     }

    fn rotate_y(&self, theta: f64) -> Vec3 {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        Vec3 {
            x: self.x * cos_theta + self.z * sin_theta,
            y: self.y,
            z: -self.x * sin_theta + self.z * cos_theta,
        }
    }

    fn rotate_x(&self, phi: f64) -> Vec3 {
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        Vec3 {
            x: self.x,
            y: self.y * cos_phi - self.z * sin_phi,
            z: self.y * sin_phi + self.z * cos_phi,
        }
    }

    pub fn rotate(&self, theta: f64, phi: f64) -> Vec3 {
        self.rotate_x(theta).rotate_y(phi)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}