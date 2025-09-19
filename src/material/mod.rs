pub mod dielectric;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Point3;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct DefaultMaterial;

impl DefaultMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for DefaultMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
