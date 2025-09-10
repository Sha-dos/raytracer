use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vector3::new(radius, radius, radius);
        
        Sphere { center,
            radius,
            mat,
            bbox: AABB::new_points(center - rvec, center + rvec)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().length_squared();
        let half_b = Vector3::dot(&oc, &ray.get_direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
    
    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

