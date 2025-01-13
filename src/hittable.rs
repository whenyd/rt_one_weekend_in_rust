use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
