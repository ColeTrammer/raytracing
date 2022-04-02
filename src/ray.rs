use super::vec3::Point3;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
