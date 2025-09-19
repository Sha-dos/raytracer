use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use std::sync::Arc;

pub struct Quad {
    pub q: Point3,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
    pub mat: Arc<dyn Material>,
    pub bbox: AABB,
    pub normal: Vector3,
    pub d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vector3, v: Vector3, mat: Arc<dyn Material>) -> Self {
        let n = Vector3::cross(&u, &v);
        let normal = Vector3::unit_vector(&n);
        let d = Vector3::dot(&normal, &q);
        let w = n / Vector3::dot(&n, &n);

        let mut quad = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: AABB::new(Interval::new(0.0, 0.0), Interval::new(0.0, 0.0), Interval::new(0.0, 0.0)), // temporary
            normal,
            d,
        };

        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = AABB::new_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = AABB::new_points(self.q + self.u, self.q + self.v);
        self.bbox = AABB::new_from_aabbs(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = Vector3::dot(&self.normal, &ray.get_direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t_hit = (self.d - Vector3::dot(&self.normal, &ray.get_origin())) / denom;
        if !t.contains(t_hit) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t_hit);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vector3::dot(&self.w, &Vector3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vector3::dot(&self.w, &Vector3::cross(&self.u, &planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t_hit;
        rec.p = intersection;
        rec.mat = Arc::clone(&self.mat);
        rec.set_face_normal(ray, &self.normal);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
