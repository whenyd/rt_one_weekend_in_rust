use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        // fmax(0, radius)
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().squared_length(); // 简化了代码写法
        let h = dot(r.direction(), oc); // 降低了运算的复杂度
        let c = oc.squared_length() - self.radius * self.radius; // 简化了代码写法
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let p = r.at(root);
        Some(HitRecord {
            p,
            t: root,
            normal: (p - self.center) / self.radius,
        })
    }
}
