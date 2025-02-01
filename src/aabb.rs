use crate::interval::{Interval, IntervalParameter};
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

pub enum AABBParameter {
    Default { x: Interval, y: Interval, z: Interval },
    Point { a: Point3, b: Point3 },
    Box { box1: AABB, box2: AABB },
}

impl AABB {
    pub fn new(param: AABBParameter) -> Self {
        match param {
            AABBParameter::Default { x, y, z } => {
                Self { x, y, z }
            }
            AABBParameter::Point { a, b } => {
                let x_param = IntervalParameter::Range { min: a.x().min(b.x()), max: a.x().max(b.x()) };
                let y_param = IntervalParameter::Range { min: a.y().min(b.y()), max: a.y().max(b.y()) };
                let z_param = IntervalParameter::Range { min: a.z().min(b.z()), max: a.z().max(b.z()) };

                Self {
                    x: Interval::new(x_param),
                    y: Interval::new(y_param),
                    z: Interval::new(z_param),
                }
            }
            AABBParameter::Box { box1, box2 } => {
                Self {
                    x: Interval::new(IntervalParameter::EncloseInterval { a: box1.x, b: box2.x }),
                    y: Interval::new(IntervalParameter::EncloseInterval { a: box1.y, b: box2.y }),
                    z: Interval::new(IntervalParameter::EncloseInterval { a: box1.z, b: box2.z }),
                }
            }
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);

            let i = axis as usize;
            let t0 = (ax.min - ray_orig[i]) / ray_dir[i];
            let t1 = (ax.max - ray_orig[i]) / ray_dir[i];

            if t0 < t1 {
                ray_t.min = t0.max(ray_t.min);
                ray_t.max = t1.min(ray_t.max);
            } else {
                ray_t.min = t1.max(ray_t.min);
                ray_t.max = t0.min(ray_t.max);
            }

            if ray_t.max < ray_t.min {
                return false;
            }
        }

        true
    }
}
