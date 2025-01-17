use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Scattered {
    pub ray: Ray,           // 散射后产生的光线, 或者说吸收了入射光线
    pub attenuation: Color, // 光线的衰减, 具体的材料用Albedo
}

impl Scattered {
    /// 创建 Scattered 对象.
    ///
    /// Scattered 用于描述光线和材质的相互作用, 对于我们的程序, 材料需要做两件事:
    ///
    /// 1. 产生散射光线（或者说它吸收了入射光线）;
    /// 2. 如果发生了散射, 光线应衰减多少. 我们将使用术语`albedo`来描述光线的衰减.
    ///
    /// `albedo`即反照率（拉丁语"白色"）, 在所有情况下, 它都用于定义某种形式的分数反射率（reflectance）.
    /// 反照率会随着材料颜色的变化而变化, 并且也会随着入射光线的方向而变化（例如玻璃材料）.
    fn new(ray: Ray, attenuation: Color) -> Self {
        Self { ray, attenuation }
    }
}

pub trait Material {
    /// 对于入射光线和击中点, 计算衰减和散射.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattered>;
}