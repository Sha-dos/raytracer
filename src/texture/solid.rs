use crate::color::Color;
use crate::texture::Texture;

pub struct SolidTexture {
    albedo: Color
}

impl SolidTexture {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &crate::vector::Point3) -> Color {
        self.albedo
    }
}