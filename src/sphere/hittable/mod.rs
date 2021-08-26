pub mod ray;

use ray::Ray;
use ray::vec3::{Vec3, Point3, Colour};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            p: p,
            normal: normal,
            t: t,
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
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool; 
}