use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::texture::solid::SolidTexture;
use crate::vector::Vector3;
use std::sync::Arc;

/// Isotropic material for volumetric scattering (used in constant medium)
pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidTexture::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(hit_record.p, Vector3::random_unit_vector());
        *attenuation = self
            .texture
            .value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}
