use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::transform::Rotation;
use std::sync::Arc;

pub struct RotateY {
    object: Arc<dyn Hittable>,
    rotation: Rotation,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let rotation = Rotation::rotate_y(angle_degrees);

        let bbox = object.bbox();
        let min = bbox.min();
        let max = bbox.max();

        let mut new_min = [f64::INFINITY; 3];
        let mut new_max = [f64::NEG_INFINITY; 3];

        // Check all 8 corners of the bounding box
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * max.x() + (1 - i) as f64 * min.x();
                    let y = j as f64 * max.y() + (1 - j) as f64 * min.y();
                    let z = k as f64 * max.z() + (1 - k) as f64 * min.z();

                    let tester = rotation.transform_point(&crate::vector::Point3::new(x, y, z));

                    for c in 0..3 {
                        new_min[c] = new_min[c].min(tester.get(c));
                        new_max[c] = new_max[c].max(tester.get(c));
                    }
                }
            }
        }

        let new_bbox = AABB::new_points(
            crate::vector::Point3::new(new_min[0], new_min[1], new_min[2]),
            crate::vector::Point3::new(new_max[0], new_max[1], new_max[2]),
        );

        Self {
            object,
            rotation,
            bbox: new_bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let origin = self.rotation.inverse_transform_point(&ray.get_origin());
        let direction = self.rotation.inverse_transform_vector(&ray.get_direction());
        let rotated_ray = Ray::new(origin, direction);

        if !self.object.hit(&rotated_ray, t, rec) {
            return false;
        }

        rec.p = self.rotation.transform_point(&rec.p);
        rec.normal = self.rotation.transform_vector(&rec.normal);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateX {
    object: Arc<dyn Hittable>,
    rotation: Rotation,
    bbox: AABB,
}

impl RotateX {
    pub fn new(object: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let rotation = Rotation::rotate_x(angle_degrees);

        let bbox = object.bbox();
        let min = bbox.min();
        let max = bbox.max();

        let mut new_min = [f64::INFINITY; 3];
        let mut new_max = [f64::NEG_INFINITY; 3];

        // Check all 8 corners of the bounding box
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * max.x() + (1 - i) as f64 * min.x();
                    let y = j as f64 * max.y() + (1 - j) as f64 * min.y();
                    let z = k as f64 * max.z() + (1 - k) as f64 * min.z();

                    let tester = rotation.transform_point(&crate::vector::Point3::new(x, y, z));

                    for c in 0..3 {
                        new_min[c] = new_min[c].min(tester.get(c));
                        new_max[c] = new_max[c].max(tester.get(c));
                    }
                }
            }
        }

        let new_bbox = AABB::new_points(
            crate::vector::Point3::new(new_min[0], new_min[1], new_min[2]),
            crate::vector::Point3::new(new_max[0], new_max[1], new_max[2]),
        );

        Self {
            object,
            rotation,
            bbox: new_bbox,
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let origin = self.rotation.inverse_transform_point(&ray.get_origin());
        let direction = self.rotation.inverse_transform_vector(&ray.get_direction());
        let rotated_ray = Ray::new(origin, direction);

        if !self.object.hit(&rotated_ray, t, rec) {
            return false;
        }

        rec.p = self.rotation.transform_point(&rec.p);
        rec.normal = self.rotation.transform_vector(&rec.normal);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}

pub struct RotateZ {
    object: Arc<dyn Hittable>,
    rotation: Rotation,
    bbox: AABB,
}

impl RotateZ {
    pub fn new(object: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let rotation = Rotation::rotate_z(angle_degrees);

        let bbox = object.bbox();
        let min = bbox.min();
        let max = bbox.max();

        let mut new_min = [f64::INFINITY; 3];
        let mut new_max = [f64::NEG_INFINITY; 3];

        // Check all 8 corners of the bounding box
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * max.x() + (1 - i) as f64 * min.x();
                    let y = j as f64 * max.y() + (1 - j) as f64 * min.y();
                    let z = k as f64 * max.z() + (1 - k) as f64 * min.z();

                    let tester = rotation.transform_point(&crate::vector::Point3::new(x, y, z));

                    for c in 0..3 {
                        new_min[c] = new_min[c].min(tester.get(c));
                        new_max[c] = new_max[c].max(tester.get(c));
                    }
                }
            }
        }

        let new_bbox = AABB::new_points(
            crate::vector::Point3::new(new_min[0], new_min[1], new_min[2]),
            crate::vector::Point3::new(new_max[0], new_max[1], new_max[2]),
        );

        Self {
            object,
            rotation,
            bbox: new_bbox,
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        let origin = self.rotation.inverse_transform_point(&ray.get_origin());
        let direction = self.rotation.inverse_transform_vector(&ray.get_direction());
        let rotated_ray = Ray::new(origin, direction);

        if !self.object.hit(&rotated_ray, t, rec) {
            return false;
        }

        rec.p = self.rotation.transform_point(&rec.p);
        rec.normal = self.rotation.transform_vector(&rec.normal);

        true
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
