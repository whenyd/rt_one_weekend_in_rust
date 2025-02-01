use std::rc::Rc;

use crate::aabb::{AABB, AABBParameter};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval, IntervalParameter};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut this: Self = Default::default();
        this.add(object);
        this
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::new(AABBParameter::Box { box1: self.bbox, box2: object.bounding_box() });
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max;

        // 使用 iter() 遍历，以确保只获取不可变引用
        for object in self.objects.iter() {
            if let Some(temp) = object.hit(r, Interval::new(IntervalParameter::Range { min: ray_t.min, max: closest_so_far })) {
                hit_anything = true;
                closest_so_far = temp.t;
                rec = temp;
            }
        }

        // if hit_anything {
        //     Some(rec)
        // } else {
        //     None
        // }
        hit_anything.then(|| rec)
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
