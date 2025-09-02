use std::ops::Sub;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::sphere::Sphere;
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
    
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
    
    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);
        if sphere.hit(&self, /* f64 */, /* f64 */, /* HitRecord */) {
            
        }

        let unit_direction: Vector3 = Vector3::from(self.get_direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}