use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Point3;

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a (dyn Material + Sync + 'a),
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a (dyn Material + Sync + 'a)) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = r.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(
            r,
            point,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
