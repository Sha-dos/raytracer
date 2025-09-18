use image::{ImageBuffer, Rgb, RgbImage};
use std::env;
use std::path::Path;

pub struct Image {
    data: Option<RgbImage>,
    width: u32,
    height: u32,
}

impl Image {
    /// Creates a new empty image
    pub fn new() -> Self {
        Self {
            data: None,
            width: 0,
            height: 0,
        }
    }

    /// Creates a new image by loading from the specified file
    /// Searches in multiple locations similar to the C++ version
    pub fn from_file(image_filename: &str) -> Self {
        let mut image = Self::new();

        // Hunt for the image file in some likely locations
        let locations = [
            // Check RTW_IMAGES environment variable first
            env::var("RT_IMAGES")
                .ok()
                .map(|dir| format!("{}/{}", dir, image_filename)),
            // Then check current directory and relative paths
            Some(image_filename.to_string()),
            Some(format!("images/{}", image_filename)),
            Some(format!("../images/{}", image_filename)),
            Some(format!("../../images/{}", image_filename)),
            Some(format!("../../../images/{}", image_filename)),
            Some(format!("../../../../images/{}", image_filename)),
            Some(format!("../../../../../images/{}", image_filename)),
            Some(format!("../../../../../../images/{}", image_filename)),
        ];

        for location in locations.iter().flatten() {
            if image.load(location) {
                return image;
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        image
    }

    /// Loads image data from the given filename
    /// Returns true if the load succeeded
    pub fn load(&mut self, filename: &str) -> bool {
        match image::open(Path::new(filename)) {
            Ok(img) => {
                let rgb_img = img.to_rgb8();
                self.width = rgb_img.width();
                self.height = rgb_img.height();
                self.data = Some(rgb_img);
                true
            }
            Err(_) => false,
        }
    }

    /// Returns the width of the image, or 0 if no image is loaded
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image, or 0 if no image is loaded
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the RGB pixel data at coordinates (x, y)
    /// If there is no image data, returns magenta [255, 0, 255]
    /// Coordinates are clamped to valid ranges
    pub fn pixel_data(&self, x: i32, y: i32) -> [u8; 3] {
        // Return magenta if no image data
        if self.data.is_none() {
            return [255, 0, 255];
        }

        let data = self.data.as_ref().unwrap();

        // Clamp coordinates to valid range
        let x = Self::clamp(x, 0, self.width as i32);
        let y = Self::clamp(y, 0, self.height as i32);

        let pixel = data.get_pixel(x as u32, y as u32);
        [pixel[0], pixel[1], pixel[2]]
    }

    /// Clamps a value to the range [low, high)
    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
