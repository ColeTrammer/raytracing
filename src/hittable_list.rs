use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::ray::Ray;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + Sync + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + Sync + 'a) {
        self.objects.push(Box::new(object));
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}
