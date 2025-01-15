use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    // 记录击中正面还是反面,
    // 渲染时这对于一些对象很重要, 需要区分
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the hit record normal vector.
    ///
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    /// 也就是说法线由几何体单位化后传递进来, 因为特定的几何体往往有特殊的归一化方式,
    /// 可以降低运算复杂度.
    ///
    /// **HitRecord的** 法线可以始终指向表面的"外面", 或者始终指向入射光线.
    /// 取决于要在几何体相交时, 还是在着色时确定击中了曲面的哪一边.
    /// 在本书中, 我们的材料类型比几何类型多, 因此我们将减少工作量并在**几何时间**去确定.
    /// 这只是一个偏好问题, 您将在文献中看到这两种实现.
    ///
    /// 因此**HitRecord的法线始终指向外面**.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;

        // 光线来自表面外部, 即front face,
        // 因此击中点的法线就是 outward_normal
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
