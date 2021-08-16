use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

//TODO: needs to convert to the Result type to avoid panic at divided by zero error
impl Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    p: Point,
}

impl Vector {
    pub fn new(p: Point) -> Self {
        Self { p: p }
    }

    pub fn point(self) -> Point {
        self.p
    }

    pub fn length(self) -> f32 {
        (self * self).sqrt()
    }

    pub fn cross(v1: Vector, v2: Vector) -> Vector {
        let x1 = v1.point().x();
        let y1 = v1.point().y();
        let z1 = v1.point().z();

        let x2 = v2.point().x();
        let y2 = v2.point().y();
        let z2 = v2.point().z();

        let p = Point::new(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2);
        Vector::new(p)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.point() + other.point())
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.point() - other.point())
    }
}

impl Mul for Vector {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        let p1 = self.point();
        let p2 = other.point();
        p1.x() * p2.x() + p1.y() * p2.y() + p1.z() * p2.z()
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.point() * rhs)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(self * rhs.point())
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn multiply_point_test() {
        let p = Point::new(2.0, -1.0, 10.0);
        assert_eq!(
            Point::new(4.0, -2.0, 20.0),
            2.0 * Point::new(2.0, -1.0, 10.0)
        );
    }

    #[test]
    fn point_multiply_test() {
        let p = Point::new(2.0, -1.0, 10.0);
        assert_eq!(
            Point::new(4.0, -2.0, 20.0),
            Point::new(2.0, -1.0, 10.0) * 2.0
        );
    }

    #[test]
    fn multiply_vector_test() {
        let p = Point::new(2.0, -1.0, 10.0);
        let v = Vector::new(p);
        let p_new = Point::new(4.0, -2.0, 20.0);
        assert_eq!(Vector::new(p_new), 2.0 * v);
    }

    #[test]
    fn vector_multiply_test() {
        let p = Point::new(2.0, -1.0, 10.0);
        let v = Vector::new(p);
        let p_new = Point::new(4.0, -2.0, 20.0);
        assert_eq!(Vector::new(p_new), v * 2.0);
    }

    #[test]
    fn vector_scalar_multiply_test() {
        let p1 = Point::new(2.0, -1.0, 3.0);
        let v1 = Vector::new(p1);
        let p2 = Point::new(-2.0, -3.0, -1.0);
        let v2 = Vector::new(p2);
        assert_eq!(-4.0, v1 * v2);
    }

    #[test]
    fn vector_squared_length_test() {
        let p = Point::new(2.0, -1.0, 3.0);
        let v = Vector::new(p);
        assert_eq!(14.0, v * v);
    }

    #[test]
    fn vector_length_test() {
        let p = Point::new(3.0, 0.0, 0.0);
        let v = Vector::new(p);
        assert_eq!(3.0, v.length());
    }
}
