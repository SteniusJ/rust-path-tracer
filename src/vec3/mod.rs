use std::ops;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn empty() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    fn len(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z + self.z)
    }
    fn sqrt_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn to_normalized(&self) -> Vec3 {
        let k: f32 = 1.0 / self.len();
        *self * k
    }
    fn into_normalized(&mut self) {
        let k: f32 = 1.0 / self.len();
        *self *= k;
    }
    fn to_color(&self) -> Color {
        let normalized = self.to_normalized();
        Color {
            r: (normalized.x * 255.0) as u8,
            g: (normalized.y * 255.0) as u8,
            b: (normalized.z * 255.0) as u8
        }
    }
    fn dot(&self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: -(self.x * v.z - self.z * v.x),
            z: self.x * v.y - self.y * v.x
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
