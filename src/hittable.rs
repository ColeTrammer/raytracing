use super::material::Material;
use super::ray::Ray;
use super::vec3::Point3;
use super::vec3::Vec3;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        r: &Ray,
        point: Point3,
        outward_normal: Vec3,
        t: f64,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
