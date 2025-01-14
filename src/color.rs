use std::io::Write;

use crate::interval::Interval;

use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write) -> std::io::Result<()> {
        // color 是多个像素求平均的结果, 所以需要 clamp 确保范围正确
        let intensity = Interval::new(0.000, 0.999);

        write!(out, "{} {} {}\n",
               (256.0 * intensity.clamp(self.x())) as i32,
               (256.0 * intensity.clamp(self.y())) as i32,
               (256.0 * intensity.clamp(self.z())) as i32)
    }
}
