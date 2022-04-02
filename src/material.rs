use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Color;
use super::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        Some((self.albedo, Ray::new(hit.point, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.direction.unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction.dot(&hit.normal) <= 0.0 {
            None
        } else {
            Some((self.albedo, scattered))
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let refaction_ratio = if hit.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refaction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refaction_ratio) > rand::random::<f64>()
        {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refaction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(hit.point, direction)))
    }
}
