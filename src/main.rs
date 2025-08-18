use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use anyhow::Result;
use crate::color::Color;

mod color;

#[tokio::main]
async fn main() -> Result<()> {
    let image_width = 256;
    let image_height = 256;
    
    let mut file = File::create("image.ppm").await?;
    
    file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).await?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.;

            let color = Color::new(r, g, b);
            color.write_color(&mut file).await?;
        }
    }
    
    Ok(())
}
