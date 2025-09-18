use crate::color::Color;
use crate::texture::Texture;
use crate::texture::solid::SolidTexture;
use crate::vector::Point3;

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn new_colors(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: Box::new(SolidTexture::new(even)),
            odd: Box::new(SolidTexture::new(odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (p.x() * self.inv_scale).floor() as i32;
        let y = (p.y() * self.inv_scale).floor() as i32;
        let z = (p.z() * self.inv_scale).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
