use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use anyhow::Result;
use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::hittable::sphere::Sphere;
use crate::vector::Point3;

mod color;
mod ray;
mod vector;
mod hittable;
mod interval;
mod camera;

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = HittableList::new();
    
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    
    let mut camera = Camera::new();
    
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    
    camera.render(world).await?;

    Ok(())
}
