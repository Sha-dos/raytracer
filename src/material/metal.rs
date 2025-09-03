use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &crate::hittable::HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vector3::reflect(&ray_in.get_direction(), &hit_record.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vector3::random_unit_vector());
        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo;
        
        Vector3::dot(&reflected, &hit_record.normal) > 0.0
    }
}