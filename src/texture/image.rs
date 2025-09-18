use crate::color::Color;
use crate::image::Image;
use crate::interval::Interval;
use crate::texture::Texture;
use crate::vector::Point3;

pub struct ImageTexture {
    image: Image,
}

impl ImageTexture {
    pub fn new(image: Image) -> Self {
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = Interval::new(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
