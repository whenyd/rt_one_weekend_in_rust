use std::io::{stdout, Write};

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, unit_vector, Vec3};

mod vec3;
mod color;
mod ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    // 只需要最近的交点, 所以取较小的解
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // 求法线: 交点 - 圆心
        let N = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        // map normal to color
        return 0.5 * Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }

    let unit_direction = unit_vector(r.direction());

    // 根据光线的单位方向向量的 y 进行混合
    let a = 0.5 * (unit_direction.y() + 1.0); // a的范围为 [0, 1]

    // a越小, 越靠近地面, 越白; 反之, 越蓝
    (1.0 - a) * Color::new(1.0, 1.0, 1.0)
        + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    /* Image */
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // 计算图像高度，并确保至少为1。
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    /* Camera */
    // 视口宽度小于1是可以的，因为它们是实值。
    let viewport_height = 2.0;
    // 视口宽度要计算, 而不能直接取图像宽度, 两者不同
    // 一方因为面图像高度会向下取整, 这会增加ratio; 另一方面因为图像高度最小为1
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0;
    let camera_center = Point3::default();

    // 计算垂直与视口边缘的向量(世界坐标)
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // uv 方向像素间的间隔
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // 左上角(世界坐标)
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left
        + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;


    /* Render */
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc
                + i as f64 * pixel_delta_u
                + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            pixel_color.write_color(&mut stdout().lock()).unwrap();
        }
    }
    eprintln!("\rDone!                         ");
}
