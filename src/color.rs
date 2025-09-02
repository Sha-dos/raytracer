use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use crate::vector::Vector3;

pub type Color = Vector3;

impl Color {
    pub async fn write_color(&self, file: &mut File) -> Result<()> {
        let ir = (255.999 * self.x()) as i32;
        let ig = (255.999 * self.y()) as i32;
        let ib = (255.999 * self.z()) as i32;
        
        file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).await?;
        
        Ok(())
    }
}
