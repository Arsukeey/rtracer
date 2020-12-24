use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new_empty(),
            normal: Vec3::new_empty(),
            t: 0.0,
            front_face: true,
            material: Rc::new(Box::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)))),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
