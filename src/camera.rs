use crate::color::Color;
use crate::hittable::{HitRecord, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use anyhow::Result;
use rand::random_range;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::stdout;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct CameraData {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    defocus_angle: f64,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
    background: Color,
}

#[derive(Clone)]
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
    pub background: Color,

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
            focus_dist: 10.0,
            background: Color::new(1.0, 1.0, 1.0),

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

    fn color(&self, r: &Ray, world: &HittableList, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter = attenuation * self.color(&scattered, world, depth - 1);

        color_from_emission + color_from_scatter
    }

    pub fn render(&mut self, world: HittableList) -> Result<()> {
        self.initialize();

        let cores = num_cpus::get() as i32;

        println!(
            "Rendering a {}x{} image with {} samples per pixel using {} cores",
            self.image_width, self.image_height, self.samples_per_pixel, cores
        );
        println!();

        let pixels = Arc::new(Mutex::new(vec![
            Color::new(0.0, 0.0, 0.0);
            (self.image_width * self.image_height)
                as usize
        ]));
        let world = Arc::new(world);

        let mut task_ranges = Vec::new();
        let rows_per_task = (self.image_height + cores - 1) / cores;
        let mut actual_task_count = 0;

        for task_id in 0..cores {
            let start_row = task_id * rows_per_task;
            let end_row = ((task_id + 1) * rows_per_task).min(self.image_height);

            if start_row >= self.image_height {
                break;
            }

            task_ranges.push((start_row, end_row));
            actual_task_count += 1;
        }

        let actual_rows_per_task: Vec<i32> =
            task_ranges.iter().map(|(start, end)| end - start).collect();
        let progress_tracker = Arc::new(ProgressTracker::new(
            actual_task_count as usize,
            actual_rows_per_task,
            self.image_width,
        ));

        let mut handles = Vec::new();

        for (task_id, (start_row, end_row)) in task_ranges.into_iter().enumerate() {
            let pixels_clone = Arc::clone(&pixels);
            let world_clone = Arc::clone(&world);
            let progress_clone = Arc::clone(&progress_tracker);

            let camera_data = CameraData {
                image_width: self.image_width,
                image_height: self.image_height,
                samples_per_pixel: self.samples_per_pixel,
                max_depth: self.max_depth,
                pixel_samples_scale: self.pixel_samples_scale,
                center: self.center,
                pixel00_loc: self.pixel00_loc,
                pixel_delta_u: self.pixel_delta_u,
                pixel_delta_v: self.pixel_delta_v,
                defocus_angle: self.defocus_angle,
                defocus_disk_u: self.defocus_u,
                defocus_disk_v: self.defocus_v,
                background: self.background,
            };

            let handle = thread::spawn(move || {
                Self::render_slice(
                    camera_data,
                    world_clone,
                    pixels_clone,
                    start_row,
                    end_row,
                    task_id,
                    progress_clone,
                )
            });

            handles.push(handle);
        }

        // Progress monitoring in the main thread
        let progress_monitor = {
            let tracker_clone = Arc::clone(&progress_tracker);
            thread::spawn(move || {
                loop {
                    tracker_clone.print_progress();
                    thread::sleep(Duration::from_millis(500));
                }
            })
        };

        for handle in handles {
            handle.join().unwrap()?;
        }

        progress_monitor.thread().unpark(); // Stop the progress monitor

        progress_tracker.print_final();

        println!("\nWriting image to file...");
        let file = File::create("image.ppm")?;
        let mut writer = BufWriter::new(file);
        writer
            .write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;

        let pixels_guard = pixels.lock().unwrap();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_index = (j * self.image_width + i) as usize;
                pixels_guard[pixel_index].write_color(&mut writer)?;
            }
        }

        println!("Done!");
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

    fn render_slice(
        camera_data: CameraData,
        world: Arc<HittableList>,
        pixels: Arc<Mutex<Vec<Color>>>,
        start_row: i32,
        end_row: i32,
        task_id: usize,
        progress_tracker: Arc<ProgressTracker>,
    ) -> Result<()> {
        let mut pixels_completed = 0;

        for j in start_row..end_row {
            for i in 0..camera_data.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..camera_data.samples_per_pixel {
                    let r = Self::get_ray_static(&camera_data, i as f64, j as f64);
                    pixel_color +=
                        Self::color_static(&camera_data, &r, &world, camera_data.max_depth);
                }

                let final_color = camera_data.pixel_samples_scale * pixel_color;
                let pixel_index = (j * camera_data.image_width + i) as usize;

                // Write to shared pixel buffer
                {
                    let mut pixels_guard = pixels.lock().unwrap();
                    pixels_guard[pixel_index] = final_color;
                }

                // Update progress after every 10 pixels to reduce overhead
                pixels_completed += 1;
                if pixels_completed % 10 == 0 {
                    progress_tracker.update_progress(task_id, pixels_completed);
                }
            }
        }

        // Final progress update
        progress_tracker.update_progress(task_id, pixels_completed);
        Ok(())
    }

    // Static helper methods for parallel rendering
    fn get_ray_static(camera_data: &CameraData, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = camera_data.pixel00_loc
            + ((i + offset.x()) * camera_data.pixel_delta_u)
            + ((j + offset.y()) * camera_data.pixel_delta_v);

        let ray_origin = if camera_data.defocus_angle <= 0.0 {
            camera_data.center
        } else {
            Self::defocus_disk_sample_static(camera_data)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample_static(camera_data: &CameraData) -> Point3 {
        let p = Vector3::random_in_unit_disk();
        camera_data.center + p.x() * camera_data.defocus_disk_u + p.y() * camera_data.defocus_disk_v
    }

    fn color_static(camera_data: &CameraData, r: &Ray, world: &HittableList, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return camera_data.background;
        }

        let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter =
            attenuation * Self::color_static(camera_data, &scattered, world, depth - 1);

        color_from_emission + color_from_scatter
    }
}

// Progress tracking structure for parallel rendering
#[derive(Clone)]
struct ProgressTracker {
    task_progress: Arc<Vec<AtomicUsize>>,
    total_pixels_per_task: Vec<usize>,
    task_count: usize,
    start_time: Instant,
}

impl ProgressTracker {
    fn new(task_count: usize, rows_per_task: Vec<i32>, image_width: i32) -> Self {
        let task_progress = Arc::new(
            (0..task_count)
                .map(|_| AtomicUsize::new(0))
                .collect::<Vec<_>>(),
        );
        let total_pixels_per_task = rows_per_task
            .iter()
            .map(|&rows| (rows * image_width) as usize)
            .collect();

        Self {
            task_progress,
            total_pixels_per_task,
            task_count,
            start_time: Instant::now(),
        }
    }

    fn update_progress(&self, task_id: usize, pixels_completed: usize) {
        self.task_progress[task_id].store(pixels_completed, Ordering::Relaxed);
    }

    fn print_progress(&self) {
        let elapsed = self.start_time.elapsed();
        let mut total_completed = 0;
        let mut total_pixels = 0;

        print!("\r");

        for (task_id, progress) in self.task_progress.iter().enumerate() {
            let completed = progress.load(Ordering::Relaxed);
            let total = self.total_pixels_per_task[task_id];
            let percentage = if total > 0 {
                (completed * 100) / total
            } else {
                0
            };

            total_completed += completed;
            total_pixels += total;

            if task_id > 0 {
                print!(" | ");
            }
            print!("T{}: {:3}%", task_id + 1, percentage);
        }

        let overall_percentage = if total_pixels > 0 {
            (total_completed * 100) / total_pixels
        } else {
            0
        };
        print!(
            " | Overall: {:3}% | {:02}:{:02}",
            overall_percentage,
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60
        );

        stdout().flush().unwrap_or(());
    }

    fn print_final(&self) {
        let total_time = self.start_time.elapsed();
        let total_pixels: usize = self.total_pixels_per_task.iter().sum();
        let total_completed: usize = self
            .task_progress
            .iter()
            .map(|p| p.load(Ordering::Relaxed))
            .sum();

        println!("\nRendering complete!");
        println!("Total pixels: {}", total_pixels);
        println!("Pixels rendered: {}", total_completed);
        println!(
            "Elapsed time: {:02}:{:02}",
            total_time.as_secs() / 60,
            total_time.as_secs() % 60
        );
    }
}
