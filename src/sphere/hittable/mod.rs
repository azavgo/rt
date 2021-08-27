pub mod ray;

use ray::vec3::{Colour, Point3, Vec3};
use ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }

    pub fn p(self) -> Point3 {
        self.p
    }

    pub fn normal(self) -> Vec3 {
        self.normal
    }

    pub fn t(self) -> f64 {
        self.t
    }

    pub fn front_face(self) -> bool {
        self.front_face
    }

    pub fn set_face_normal(mut self, r: Ray, outward_normal: Vec3) -> Self {
        if r.direction() * outward_normal > 0.0 {
            Self::new(self.p(), -1.0 * outward_normal, self.t(), false)
        } else {
            Self::new(self.p(), outward_normal, self.t(), true)
        }
    }

}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}
