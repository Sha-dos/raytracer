use std::ops::Sub;
use crate::color::Color;
use crate::vector::{Point3, Vector3};

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction
        }
    }
    
    pub fn get_origin(&self) -> Vector3 {
        self.origin
    }
    
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }
    
    pub fn color(&self) -> Color {
        if (self.hit_sphere(&Point3::new(0., 0., -1.), 0.5)) {
            return Color::new(1., 0., 0.);
        }

        let unit_direction: Vector3 = Vector3::from(self.get_direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
    
    fn hit_sphere(&self, center: &Vector3, radius: f64) -> bool {
        let oc = self.origin - *center;
        let a = Vector3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vector3::dot(&oc, &self.direction);
        let c = Vector3::dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4. * a * c;
        discriminant >= 0.
    }
}