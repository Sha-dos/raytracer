use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use anyhow::Result;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::hittable::sphere::Sphere;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::vector::Point3;

mod color;
mod ray;
mod vector;
mod hittable;
mod interval;
mod camera;
mod material;

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = HittableList::new();
    
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1. / 1.5));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));
    
    world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0),  0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Point3::new( 1.0, 0.0, -1.0), 0.5, material_right)));
    
    let mut camera = Camera::new();
    
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    
    camera.render(world).await?;

    Ok(())
}
