use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}

type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e0: e0,
            e1: e1,
            e2: e2,
        }
    }

    pub fn e() -> Self {
        Self::new(0f64, 0f64, 0f64)
    }

    pub fn x(self) -> f64 {
        self.e0
    }

    pub fn y(self) -> f64 {
        self.e1
    }

    pub fn z(self) -> f64 {
        self.e2
    }

    pub fn length(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self::new(
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        )
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u * v
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self::new(self.x() + other, self.y() + other, self.z() + other)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.x(), self + other.y(), self + other.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.x() - other, self.y() - other, self.z() - other)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self - other.x(), self - other.y(), self - other.z())
    }
}

impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x() / other, self.y() / other, self.z() / other)
    }
}
