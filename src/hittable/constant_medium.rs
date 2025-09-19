use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::isotropic::Isotropic;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vector::Vector3;
use rand::random_range;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<Isotropic>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Find the entry and exit points of the ray through the boundary
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        // Use a very wide interval to find all intersections
        let mut interval_universe = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

        if !self.boundary.hit(ray, &mut interval_universe, &mut rec1) {
            return false;
        }

        // Find the second intersection (exit point)
        let mut interval_after_first = Interval::new(rec1.t + 0.0001, f64::INFINITY);
        if !self.boundary.hit(ray, &mut interval_after_first, &mut rec2) {
            return false;
        }

        // Clamp the intersection points to the valid ray parameter range
        if rec1.t < t.min {
            rec1.t = t.min;
        }
        if rec2.t > t.max {
            rec2.t = t.max;
        }

        // If the ray doesn't pass through the medium, return false
        if rec1.t >= rec2.t {
            return false;
        }

        // Ensure we don't have negative ray parameters
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.get_direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        // Calculate the distance to the scattering event using exponential distribution
        let hit_distance = self.neg_inv_density * random_range(0f64..1f64).ln();

        // If the scattering event is beyond the boundary, no hit
        if hit_distance > distance_inside_boundary {
            return false;
        }

        // Calculate the hit point
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        // Set arbitrary normal and front_face (not meaningful for volumes)
        rec.normal = Vector3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();

        true
    }

    fn bbox(&self) -> &AABB {
        self.boundary.bbox()
    }
}
