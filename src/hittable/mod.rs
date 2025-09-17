pub mod bvh_node;
pub mod sphere;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use std::sync::Arc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool;
    fn bbox(&self) -> &AABB;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: Arc::new(DefaultMaterial::new()),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = Vector3::dot(&ray.get_direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::new();
            let mut temp_interval = Interval::new(t.min, closest_so_far);
            if object.hit(ray, &mut temp_interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new_empty(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = AABB::new_from_aabbs(&self.bbox, &object.bbox());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::new();
            let mut temp_interval = Interval::new(t.min, closest_so_far);
            if object.hit(ray, &mut temp_interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
