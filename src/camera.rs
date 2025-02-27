use std::io::{stdout, Write};

use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{degrees_to_radians, INFINITY, random};
use crate::vec3::{cross, Point3, random_in_unit_disk, unit_vector, Vec3};

pub struct Camera {
    // 通过 new 赋于默认值
    pub aspect_ratio: f64,       // Ratio of image width over height
    pub image_width: i32,        // Rendered image width in pixel count
    pub samples_per_pixel: i32,  // Count of random samples for each pixel
    pub max_depth: i32,          // Maximum number of ray bounces into scene

    pub vfov: f64,               // 垂直视场, 单位度
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,               // 相机的 view up 方向

    pub defocus_angle: f64,
    pub focus_dist: f64,

    // 在 initialize 中计算
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    // 控制散焦椭圆的大小
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}


impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            // private
            image_height: 0,
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),

            u: Default::default(),
            v: Default::default(),
            w: Default::default(),

            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        /* Render */
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            // 多一个空格, 当行数从3位数变成2位数时确保清空缓存
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            stdout().flush().unwrap();
            for i in 0..self.image_width {
                // msaa 在像素周围进行重复采样, 使得边缘过渡更平滑, 非边缘部分更加均匀, 从而提升像素质量
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                pixel_color /= self.samples_per_pixel as f64;
                pixel_color.write_color(&mut stdout().lock()).unwrap();
            }
        }
        eprintln!("\rDone!                         ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and
        // directed at a randomly sampled point around the pixel location i, j.

        // msaa: 终点随机波动, 在像素周围的square随机采样
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        // defocus blur: 起点随机波动
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn initialize(&mut self) {
        /* Image */
        // 计算图像高度，并确保至少为1。
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        /* Camera */
        let camera_center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist; // 假设成像平面始终在焦平面上
        // 视口宽度要计算, 而不能直接取图像宽度, 两者不同
        // 一方因为面图像高度会向下取整, 这会增加ratio; 另一方面因为图像高度最小为1
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // 计算垂直与视口边缘的向量(世界坐标)
        let viewport_u = viewport_width * self.u;    // 沿着视口水平方向的投影
        let viewport_v = viewport_height * -self.v;  // 沿着视口垂直边向下的投影

        // uv 方向像素间的间隔
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // 左上角(世界坐标)
        let viewport_upper_left = camera_center
            - self.focus_dist * self.w
            - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left
            + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        self.image_height = image_height;
        self.center = camera_center;
        self.pixel00_loc = pixel00_loc;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Returns the color for a given scene ray.
    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        // t的最小值略大于0, 忽略很近的命中点, 因为可能时浮点计算误差产生的
        match world.hit(r, Interval::new(0.001, INFINITY)) {
            Some(rec) => {
                // fixme 循环引用mat
                if let Some(mat) = rec.mat.clone() {
                    if let Some(scattered) = mat.scatter(r, &rec) {
                        return scattered.attenuation * Self::ray_color(&scattered.ray, depth - 1, world);
                    }
                }
                return Color::default();
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
        Vec3::new(random() - 0.5, random() - 0.5, 0.0)
    }
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
