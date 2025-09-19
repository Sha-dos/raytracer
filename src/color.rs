use crate::interval::Interval;
use crate::vector::Vector3;
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type Color = Vector3;

impl Color {
    pub fn write_color(&self, file: &mut BufWriter<File>) -> Result<()> {
        let intensity = Interval::new(0., 0.999);

        // Apply gamma correction
        let r = Color::linear_to_gamma(self.x());
        let g = Color::linear_to_gamma(self.y());
        let b = Color::linear_to_gamma(self.z());

        let r = (256. * intensity.clamp(r)) as i32;
        let g = (256. * intensity.clamp(g)) as i32;
        let b = (256. * intensity.clamp(b)) as i32;

        file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;

        Ok(())
    }

    pub fn linear_to_gamma(linear: f64) -> f64 {
        if linear > 0. { linear.sqrt() } else { 0. }
    }
}
