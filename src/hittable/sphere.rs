use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
        let oc = self.center -  ray.get_origin();
        let a = ray.get_direction().length_squared();
        let h = Vector3::dot(&ray.get_direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = ray.at(rec.t);
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        
        true
    }
}