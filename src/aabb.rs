use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Point3;

pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    
    pub fn new_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() < b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        
        let y = if a.y() < b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        
        let z = if a.z() < b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        
        Self { x, y, z }
    }
    
    pub fn axis_interval(&self, axis: i32) -> &Interval {
        match axis {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
    
    pub fn hit(&self, ray: &Ray, t: &mut Interval) -> bool {
        let origin = ray.get_origin();
        let direction = ray.get_direction();
        
        for axis in 0..3 {
            let interval = self.axis_interval(axis);
            let adinv = 1.0 / direction[axis as usize];
            
            let t0 = (interval.min - origin[axis as usize]) * adinv;
            let t1 = (interval.max - origin[axis as usize]) * adinv;
            
            if t0 < t1 {
                if t0 > t.min { t.min = t0; }
                if t1 < t.max { t.max = t1; }
            } else {
                if t1 > t.min { t.min = t1; }
                if t0 < t.max { t.max = t0; }
            }
            
            if t.max <= t.min {
                return false;
            }
        }
        
        false
    }
}