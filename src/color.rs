use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;

pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub async fn write_color(&self, file: &mut File) -> Result<()> {
        let ir = (255.999 * self.r) as i32;
        let ig = (255.999 * self.g) as i32;
        let ib = (255.999 * self.b) as i32;
        
        file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).await?;
        
        Ok(())
    }
}
