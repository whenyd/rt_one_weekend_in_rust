use std::io::Write;

use crate::interval::{Interval, IntervalParameter};

use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write) -> std::io::Result<()> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // 应用 Gamma 校正
        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        // color 是多个像素求平均的结果, 所以需要 clamp 确保范围正确
        let intensity = Interval::new(IntervalParameter::Range { min: 0.000, max: 0.999 });
        let rbytes = (256.0 * intensity.clamp(r)) as i32;
        let gbytes = (256.0 * intensity.clamp(g)) as i32;
        let bbytes = (256.0 * intensity.clamp(b)) as i32;

        write!(out, "{} {} {}\n", rbytes, gbytes, bbytes)
    }
}

/// 应用`Gamma2`逆变换, 即返回输入的平方根.
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}
