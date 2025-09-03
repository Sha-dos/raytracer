use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use crate::interval::Interval;
use crate::vector::Vector3;

pub type Color = Vector3;

impl Color {
    pub async fn write_color(&self, file: &mut File) -> Result<()> {
        let intensity = Interval::new(0., 0.999);
        
        // Apply gamma correction
        let r = (256. * intensity.clamp(self.x())) as i32;
        let g = (256. * intensity.clamp(self.y())) as i32;
        let b = (256. * intensity.clamp(self.z())) as i32;
        
        file.write(format!("{} {} {}\n", r, g, b).as_bytes()).await?;
        
        Ok(())
    }
}
