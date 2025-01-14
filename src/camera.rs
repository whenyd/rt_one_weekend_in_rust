use std::io::{stdout, Write};

use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, random_double};
use crate::vec3::{Point3, unit_vector, Vec3};

pub struct Camera {
    // 通过 new 赋于默认值
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,

    // 在 initialize 中计算
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}


impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,

            image_height: 0,
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        /* Render */
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, world);
                }

                pixel_color /= self.samples_per_pixel as f64;
                pixel_color.write_color(&mut stdout().lock()).unwrap();
            }
        }
        eprintln!("\rDone!                         ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn initialize(&mut self) {
        /* Image */
        // 计算图像高度，并确保至少为1。
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        /* Camera */
        // 视口宽度小于1是可以的，因为它们是实值。
        let viewport_height = 2.0;
        // 视口宽度要计算, 而不能直接取图像宽度, 两者不同
        // 一方因为面图像高度会向下取整, 这会增加ratio; 另一方面因为图像高度最小为1
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);
        let focal_length = 1.0;
        let camera_center = Point3::default();

        // 计算垂直与视口边缘的向量(世界坐标)
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // uv 方向像素间的间隔
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // 左上角(世界坐标)
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left
            + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        self.image_height = image_height;
        self.center = camera_center;
        self.pixel00_loc = pixel00_loc;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        match world.hit(r, Interval::new(0.0, INFINITY)) {
            Some(rec) => {
                0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
            }
            None => {
                let unit_direction = unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y() + 1.0); // a的范围为 [0, 1]
                (1.0 - a) * Color::new(1.0, 1.0, 1.0)
                    + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square() -> Vec3 {
        // 从 [0,1) 到 [-0.5, 0.5]
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
