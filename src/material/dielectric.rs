use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use rand::random_range;
use crate::texture::solid::SolidTexture;
use crate::texture::Texture;

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ir = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.get_direction().unit_vector();
        let cos_theta = f64::min(Vector3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ir * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, ir) > random_range(0f64..1f64)
        {
            Vector3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vector3::refract(&unit_direction, &hit_record.normal, ir)
        };

        *scattered = Ray::new(hit_record.p, direction);

        true
    }
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new (texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
    
    pub fn from_color (color: Color) -> Self {
        Self { texture: Arc::new(SolidTexture::new(color)) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
    
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.texture.value(u, v, p)
    }
}
