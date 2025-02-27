use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        // fmax(0, radius)
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared(); // 简化了代码写法
        let h = dot(r.direction(), oc); // 降低了运算的复杂度
        let c = oc.length_squared() - self.radius * self.radius; // 简化了代码写法
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            p,
            t: root,
            normal: outward_normal, // 法线始终指向表面"外面", 而且为单位向量
            front_face: false,
            mat: Some(Rc::clone(&self.mat)),
        };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
