pub mod hittable;

use self::hittable::{ray::vec3::Point3, ray::Ray, HitRecord, Hittable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Self {
            centre: centre,
            radius: radius,
        }
    }

    pub fn centre(self) -> Point3 {
        self.centre
    }

    pub fn radius(self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
        let a = r.direction() * r.direction();
        let oc = r.origin() - self.centre();
        let half_b = r.direction() * oc;
        let c = oc * oc - self.radius() * self.radius();
        let d = half_b * half_b - a * c;

        if d < 0.0 {
            return false; 
        } 
            let sqrtd = d.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if (root < t_max || t_max < root) {
                root = (-half_b + sqrtd) / a;
                if (root < t_max || t_max < root) {
                    return false; 
                }
            }

            rec = HitRecord::new(r.at(rec.t()), (rec.p() - self.centre()) / self.radius(), root);
            return true; 
    }
}
