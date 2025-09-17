use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(list: &HittableList) -> BVHNode {
        Self::new_from_objects(&list.objects, 0, list.objects.len())
    }

    pub fn new_from_objects(objects: &[Arc<dyn Hittable>], start: usize, end: usize) -> BVHNode {
        let mut bbox = AABB::new_empty();
        for object in objects.iter().take(end).skip(start) {
            bbox = AABB::new_from_aabbs(&bbox, &object.bbox());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!("Invalid axis"),
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            // Single object case: both children point to the same object
            let obj = objects[start].clone();
            (obj.clone(), obj)
        } else if object_span == 2 {
            // Two objects case: direct assignment
            let left_obj = objects[start].clone();
            let right_obj = objects[start + 1].clone();
            (left_obj, right_obj)
        } else {
            // Multiple objects case: sort and recursively build subtrees
            let mut sorted_objects = objects[start..end].to_vec();
            sorted_objects.sort_by(comparator);

            let mid = object_span / 2;
            let left_node = Arc::new(Self::new_from_objects(&sorted_objects, 0, mid));
            let right_node = Arc::new(Self::new_from_objects(&sorted_objects, mid, object_span));

            (
                left_node as Arc<dyn Hittable>,
                right_node as Arc<dyn Hittable>,
            )
        };

        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Ordering {
        let box_a = a.bbox();
        let box_b = b.bbox();

        if box_a.axis_interval(axis).min < box_b.axis_interval(axis).min {
            Ordering::Less
        } else if box_a.axis_interval(axis).min > box_b.axis_interval(axis).min {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t) {
            return false;
        }

        let mut temp_rec = HitRecord::new();
        let mut hit_left = self.left.hit(ray, t, &mut temp_rec);
        
        let mut right_t = t;
        if hit_left {
            right_t.max = temp_rec.t;
        }
        
        let mut right_rec = HitRecord::new();
        let hit_right = self.right.hit(ray, &mut right_t, &mut right_rec);
        
        // Choose the closest hit
        if hit_left && hit_right {
            if temp_rec.t < right_rec.t {
                *rec = temp_rec;
            } else {
                *rec = right_rec;
            }
            true
        } else if hit_left {
            *rec = temp_rec;
            true
        } else if hit_right {
            *rec = right_rec;
            true
        } else {
            false
        }
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
