use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new_with_point(a: &Point3, b: &Point3) -> Self {
        let x = if a.x() < b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) };
        let y = if a.y() < b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) };
        let z = if a.z() < b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) };
        Self { x, y, z }
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
