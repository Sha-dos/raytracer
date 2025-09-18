use crate::color::Color;
use crate::perlin::Perlin;
use crate::texture::Texture;
use crate::vector::Point3;

pub struct NoiseTexture {
    scale: f64,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(*p * self.scale)))
    }
}
