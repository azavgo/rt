pub mod vec3;
use self::vec3::{Colour, Point3};
use vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin() + t * self.direction()
    }

    pub fn ray_colour(&self) -> Colour {
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * unit_direction.y() + 1.0;
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}
