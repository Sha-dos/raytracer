use crate::color::Color;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use anyhow::Result;
use rand::random_range;
use std::io::Write;
use std::io::stdout;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,          // Vertical field of view in degrees
    pub lookfrom: Point3,   // Point camera is looking from
    pub lookat: Point3,     // Point camera is looking at
    pub vup: Vector3,       // Camera-relative "up" direction
    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance to perfect focus plane

    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    u: Vector3, // Camera frame basis vectors
    v: Vector3,
    w: Vector3,
    defocus_u: Vector3,
    defocus_v: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 0.0,

            // These will be calculated in initialize()
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            u: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(0.0, 0.0, 0.0),
            w: Vector3::new(0.0, 0.0, 0.0),
            defocus_u: Vector3::new(0.0, 0.0, 0.0),
            defocus_v: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = Vector3::cross(&self.vup, &self.w).unit_vector();
        self.v = Vector3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * (-self.v); // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_u = self.u * defocus_radius;
        self.defocus_v = self.v * defocus_radius;
    }

    pub async fn render(&mut self, world: HittableList) -> Result<()> {
        self.initialize();

        let mut file = File::create("image.ppm").await?;

        file.write(format!("P3\n{} {}\n255\n", &self.image_width, &self.image_height).as_bytes())
            .await?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            stdout().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += r.color(&world, self.max_depth);
                }

                (self.pixel_samples_scale * pixel_color)
                    .write_color(&mut file)
                    .await?;
            }
        }

        println!("\rDone.                 ");
        Ok(())
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vector3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
        Vector3::new(
            random_range(0.0..1.0) - 0.5,
            random_range(0.0..1.0) - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vector3::random_in_unit_disk();
        self.center + p.x() * self.defocus_u + p.y() * self.defocus_v
    }
}
