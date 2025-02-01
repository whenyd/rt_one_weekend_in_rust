use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::{Interval, IntervalParameter};
use crate::ray::Ray;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

pub enum BVHParameter {
    HittableList(HittableList),
    Objects { objects: Vec<Rc<dyn Hittable>>, start: usize, end: usize },
}

impl BVHNode {
    pub fn new(param: BVHParameter) -> Self {
        match param {
            BVHParameter::HittableList(list) => {
                let end = list.objects.len();
                Self::new_with_range(list.objects, 0, end)
            }
            BVHParameter::Objects { objects, start, end } =>
                Self::new_with_range(objects, start, end),
        }
    }

    fn new_with_range(objects: Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // To be implemented later.
        unimplemented!()
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut ray_t = ray_t.clone();
        if !self.bbox.hit(r, &mut ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let hit_right = self.right.hit(r, Interval::new(
            IntervalParameter::Range {
                min: ray_t.min,
                max: hit_left.as_ref().map_or(ray_t.max, |rec| rec.t),
            }));

        hit_left.or(hit_right)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

