use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use anyhow::Result;
use crate::ray::Ray;
use crate::vector::Vector3;

mod color;
mod ray;
mod vector;

#[tokio::main]
async fn main() -> Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Vector3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0., 0.);
    let viewport_v = Vector3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vector3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    let mut file = File::create("image.ppm").await?;
    
    file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()).await?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = Ray::color(&r);
            pixel_color.write_color(&mut file).await?;
        }
    }
    
    Ok(())
}
