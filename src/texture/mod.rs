pub mod checker;
pub mod image;
pub mod solid;
pub mod noise;

use crate::color::Color;
use crate::vector::Point3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
