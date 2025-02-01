use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::{AABB, AABBParameter};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::{Interval, IntervalParameter};
use crate::ray::Ray;
use crate::rtweekend::random_int;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

pub enum BVHParameter {
    HittableList(HittableList),
    Objects { objects: Vec<Rc<dyn Hittable>> },
}

impl BVHNode {
    pub fn new(param: BVHParameter) -> Self {
        let mut objects = match param {
            BVHParameter::HittableList(list) => list.objects,
            BVHParameter::Objects { objects } => objects,
        };

        Self::build_with_objects(&mut objects)
    }

    fn build_with_objects(objects: &mut [Rc<dyn Hittable>]) -> Self {
        // 1. 随机选择一个轴
        let axis = random_int(0, 2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        let object_span = objects.len();

        match object_span {
            // 只有一个元素则复制到两个子树中
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            // 两个元素则每个子树各一个
            2 => {
                left = objects[0].clone();
                right = objects[1].clone();
            }
            _ => {
                // 2. Sort the primitives
                objects.sort_by(|a, b| comparator(a, b));

                // 3. 每个子树各一半
                let mid = object_span / 2;
                left = Rc::new(BVHNode::build_with_objects(&mut objects[..mid]));
                right = Rc::new(BVHNode::build_with_objects(&mut objects[mid..]));
            }
        }

        let bbox = AABB::new(AABBParameter::Box { box1: left.bounding_box(), box2: right.bounding_box() });
        Self { left, right, bbox }
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let binding = a.bounding_box();
        let a_axis_interval = binding.axis_interval(axis);
        let binding = b.bounding_box();
        let b_axis_interval = binding.axis_interval(axis);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
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

