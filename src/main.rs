use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::hittable::bvh_node::BVHNode;
use crate::hittable::sphere::Sphere;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::vector::{Point3, Vector3};
use anyhow::Result;
use std::sync::Arc;

mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod vector;

#[tokio::main]
async fn main() -> Result<()> {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1. / 1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // let bvh_world = Arc::new(BVHNode::new(&world));
    // world.clear();
    // world.add(bvh_world);

    let mut camera = Camera::new();

    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.;

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    camera.lookfrom = Point3::new(-2., 2., 1.);
    camera.lookat = Point3::new(0., 0., -1.);
    camera.vup = Vector3::new(0., 1., 0.);

    camera.initialize();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.render(world).await?;

    Ok(())
}
