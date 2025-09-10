use std::cmp::Ordering;
use std::sync::Arc;
use rand::random_range;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;

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
            _ => panic!("Invalid axis")
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
            
            (left_node as Arc<dyn Hittable>, right_node as Arc<dyn Hittable>)
        };
        
        Self { left, right, bbox }
    }
    
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Ordering {
        let box_a = a.bbox();
        let box_b = b.bbox();
        
        if box_a.axis_interval(axis).min < box_b.axis_interval(axis).min {
            std::cmp::Ordering::Less
        } else if box_a.axis_interval(axis).min > box_b.axis_interval(axis).min {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
    
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }
    
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }
    
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t) {
            return false;
        }
        
        let left_hit = self.left.hit(&ray, t, rec);
        let interval = if left_hit { &mut Interval::new(t.min, rec.t) } else { &mut Interval::new(t.min, t.max) };
        
        let right_hit = self.right.hit(&ray, interval, rec);
        
        left_hit || right_hit
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}