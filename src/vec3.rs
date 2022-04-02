use rand::Rng;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        self.clone() - 2.0 * self.dot(normal) * normal.clone()
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-self.clone()).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self.clone() + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Self::Output {
        v.clone() * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn basics() {
        let mut v = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);

        let w = Vec3::new(15.0, 30.0, 0.0);
        assert_eq!(w.x, 15.0);
        assert_eq!(w.y, 30.0);
        assert_eq!(w.z, 0.0);

        v += w;
        assert_eq!(v.x, 15.0);
        assert_eq!(v.y, 30.0);
        assert_eq!(v.z, 0.0);

        v *= 2.0;
        assert_eq!(v.x, 30.0);
        assert_eq!(v.y, 60.0);
        assert_eq!(v.z, 0.0);

        v /= 2.0;
        assert_eq!(v.x, 15.0);
        assert_eq!(v.y, 30.0);
        assert_eq!(v.z, 0.0);

        let u = -v;
        assert_eq!(u.x, -15.0);
        assert_eq!(u.y, -30.0);
        assert_eq!(u.z, -0.0);

        let t = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(t.length(), 5.0);
        assert_eq!(t.length_squared(), 25.0);
    }
}
