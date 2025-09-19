use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::quad::{Quad, create_box};
use crate::hittable::rotate::{RotateX, RotateY, RotateZ};
use crate::hittable::sphere::Sphere;
use crate::image::Image;
use crate::material::dielectric::{Dielectric, DiffuseLight};
use crate::material::lambertian::Lambertian;
use crate::texture::checker::CheckerTexture;
use crate::texture::image::ImageTexture;
use crate::texture::noise::NoiseTexture;
use crate::vector::{Point3, Vector3};
use anyhow::Result;
use std::sync::Arc;
use crate::material::metal::Metal;

mod aabb;
mod camera;
mod color;
mod hittable;
mod image;
mod interval;
mod material;
mod perlin;
mod ray;
mod texture;
mod transform;
mod vector;

fn main() -> Result<()> {
    match 5 {
        1 => spheres()?,
        2 => quads()?,
        3 => simple_light()?,
        4 => cornell_box()?,
        5 => cornell_box_smoke()?,
        _ => (),
    }

    Ok(())
}

fn spheres() -> Result<()> {
    let mut world = HittableList::new();

    let material_ground = Arc::new(CheckerTexture::new_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let earth_texture = Arc::new(ImageTexture::new(Image::from_file("earthmap.jpg")));
    let material_center = Arc::new(Lambertian::new_texture(earth_texture));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1. / 1.5));
    // let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));
    let material_right = Arc::new(Lambertian::new_texture(Arc::new(NoiseTexture::new(40.))));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new_texture(material_ground)),
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

    camera.defocus_angle = 1.0;
    camera.focus_dist = 3.4;

    camera.lookfrom = Point3::new(-2., 2., 1.);
    camera.lookat = Point3::new(0., 0., -1.);
    camera.vup = Vector3::new(0., 1., 0.);

    camera.initialize();

    camera.render(world)?;

    Ok(())
}

fn quads() -> Result<()> {
    let mut world = HittableList::new();

    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));

    let quad = Arc::new(Quad::new(
        Point3::new(-2., -1., 1.),
        Vector3::new(0., 2., 0.),
        Vector3::new(0., 0., -2.),
        back_green,
    ));

    world.add(quad);

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.70, 0.80, 1.00);

    camera.vfov = 80.;
    camera.lookfrom = Point3::new(-5., 0., 0.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vector3::new(0., 1., 0.);

    // camera.defocus_angle = 1.0;
    // camera.focus_dist = 3.4;

    camera.initialize();

    camera.render(world)?;

    Ok(())
}

fn simple_light() -> Result<()> {
    let mut world = HittableList::new();

    let texture = Arc::new(NoiseTexture::new(5.));
    let material_ground = Arc::new(Lambertian::new_texture(texture));

    let material_ball = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.9));
    let light = Arc::new(DiffuseLight::from_color(Color::new(4., 4., 4.)));

    world.add(Arc::new(Quad::new(
        Point3::new(-500., -4., -500.),
        Vector3::new(1000., 0., 0.),
        Vector3::new(0., 0., 1000.),
        material_ground,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        2.,
        material_ball,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(-1., 3., -2.),
        Vector3::new(2., 0., 0.),
        Vector3::new(0., 0., 2.),
        light,
    )));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0., 0., 0.);

    camera.vfov = 80.;
    camera.lookfrom = Point3::new(-5., 0., 0.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.vup = Vector3::new(0., 1., 0.);

    camera.initialize();

    camera.render(world)?;

    Ok(())
}

fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    // Left wall (green)
    world.add(Arc::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        green.clone(),
    )));

    // Right wall (red)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        red.clone(),
    )));

    // Floor (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
        white.clone(),
    )));

    // Ceiling (white)
    world.add(Arc::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vector3::new(-555., 0., 0.),
        Vector3::new(0., 0., -555.),
        white.clone(),
    )));

    // Back wall (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(213., 554., 227.),
        Vector3::new(130., 0., 0.),
        Vector3::new(0., 0., 105.),
        light.clone(),
    )));


    let tall_box = create_box(
        Point3::new(265., 0., 295.),
        Point3::new(430., 330., 460.),
        white.clone(),
    );
    let tall_box_objects = tall_box.objects;
    for object in tall_box_objects {
        world.add(Arc::new(RotateY::new(object, -18.0)));
    }

    let short_box = create_box(
        Point3::new(130., 0., 65.),
        Point3::new(295., 165., 230.),
        white.clone(),
    );
    let short_box_objects = short_box.objects;
    for object in short_box_objects {
        let rotated_object = Arc::new(RotateY::new(object, 15.0));
        world.add(rotated_object);
    }

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0., 0., 0.);

    camera.vfov = 40.;
    camera.lookfrom = Point3::new(278., 278., -800.);
    camera.lookat = Point3::new(278., 278., 0.);
    camera.vup = Vector3::new(0., 1., 0.);

    camera.initialize();

    camera.render(world)?;

    Ok(())
}

fn cornell_box_smoke() -> Result<()> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    // Left wall (green)
    world.add(Arc::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        green.clone(),
    )));

    // Right wall (red)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        red.clone(),
    )));

    // Floor (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
        white.clone(),
    )));

    // Ceiling (white)
    world.add(Arc::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vector3::new(-555., 0., 0.),
        Vector3::new(0., 0., -555.),
        white.clone(),
    )));

    // Back wall (white)
    world.add(Arc::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        white.clone(),
    )));

    // Light source
    world.add(Arc::new(Quad::new(
        Point3::new(213., 554., 227.),
        Vector3::new(130., 0., 0.),
        Vector3::new(0., 0., 105.),
        light.clone(),
    )));

    let tall_box_boundary = create_box(
        Point3::new(265., 0., 295.),
        Point3::new(430., 330., 460.),
        white.clone(),
    );

    let mut rotated_tall_box = HittableList::new();
    for object in tall_box_boundary.objects {
        rotated_tall_box.add(Arc::new(RotateY::new(object, -18.0)));
    }

    world.add(Arc::new(ConstantMedium::from_color(
        Arc::new(rotated_tall_box),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let short_box_boundary = create_box(
        Point3::new(130., 0., 65.),
        Point3::new(295., 165., 230.),
        white.clone(),
    );

    let mut rotated_short_box = HittableList::new();
    for object in short_box_boundary.objects {
        rotated_short_box.add(Arc::new(RotateY::new(object, 15.0)));
    }

    world.add(Arc::new(ConstantMedium::from_color(
        Arc::new(rotated_short_box),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 250;
    camera.max_depth = 50;
    camera.background = Color::new(0., 0., 0.);

    camera.vfov = 40.;
    camera.lookfrom = Point3::new(278., 278., -800.);
    camera.lookat = Point3::new(278., 278., 0.);
    camera.vup = Vector3::new(0., 1., 0.);

    camera.initialize();

    camera.render(world)?;

    Ok(())
}
