use std::ops::Sub;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::hittable::sphere::Sphere;
use crate::interval::Interval;
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

    pub fn color(&self, world: &HittableList) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&self, Interval::new(0.0001, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = self.get_direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}