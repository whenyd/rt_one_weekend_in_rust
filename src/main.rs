use std::io::Write;
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Point3;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;
mod material;


fn main() {
    /* World */
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });
    let material_center = Rc::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) });
    let material_left = Rc::new(Metal { albedo: Color::new(0.8, 0.8, 0.8) });
    let material_right = Rc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2) });

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
