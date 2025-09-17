pub mod solid;
pub mod checker;

use crate::color::Color;
use crate::vector::Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color { Color::new(0.0, 0.0, 0.0) }
}
