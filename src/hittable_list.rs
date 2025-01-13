use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    fn new(object: Rc<dyn Hittable>) -> Self {
        Self { objects: vec![object] }
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closet_so_far = ray_tmax;

        // 使用 iter() 遍历，以确保只获取不可变引用
        for object in self.objects.iter() {
            if let Some(temp) = object.hit(r, ray_tmin, closet_so_far) {
                hit_anything = true;
                closet_so_far = temp.t;
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
}
