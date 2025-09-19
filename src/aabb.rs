use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};
use std::ops::Add;

pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut x = x;
        let mut y = y;
        let mut z = z;

        Self::pad_to_minimums(&mut x, &mut y, &mut z);

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

    pub fn new_from_aabbs(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::new_enclosing_intervals(&box0.x, &box1.x);
        let y = Interval::new_enclosing_intervals(&box0.y, &box1.y);
        let z = Interval::new_enclosing_intervals(&box0.z, &box1.z);

        Self { x, y, z }
    }

    pub fn new_empty() -> Self {
        let inf = f64::INFINITY;
        let ninf = f64::NEG_INFINITY;

        Self {
            x: Interval::new(inf, ninf),
            y: Interval::new(inf, ninf),
            z: Interval::new(inf, ninf),
        }
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
                if t0 > t.min {
                    t.min = t0;
                }
                if t1 < t.max {
                    t.max = t1;
                }
            } else {
                if t1 > t.min {
                    t.min = t1;
                }
                if t0 < t.max {
                    t.max = t0;
                }
            }

            if t.max <= t.min {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> i32 {
        let x_len = self.x.size();
        let y_len = self.y.size();
        let z_len = self.z.size();

        if x_len > y_len && x_len > z_len {
            0
        } else if y_len > z_len {
            1
        } else {
            2
        }
    }

    fn pad_to_minimums(x: &mut Interval, y: &mut Interval, z: &mut Interval) {
        let delta = 0.0001;

        if x.size() < delta { *x = x.expand(delta); }
        if y.size() < delta { *y = y.expand(delta); }
        if z.size() < delta { *z = z.expand(delta); }
    }
}

// Implement AABB + Vector3 (translating AABB by a vector offset)
impl Add<Vector3> for AABB {
    type Output = AABB;

    fn add(self, offset: Vector3) -> Self::Output {
        AABB::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

// Implement &AABB + Vector3 (translating AABB reference by a vector offset)
impl Add<Vector3> for &AABB {
    type Output = AABB;

    fn add(self, offset: Vector3) -> Self::Output {
        AABB::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl Add<AABB> for Vector3 {
    type Output = AABB;

    fn add(self, bbox: AABB) -> Self::Output {
        bbox + self
    }
}

impl Add<&AABB> for Vector3 {
    type Output = AABB;

    fn add(self, bbox: &AABB) -> Self::Output {
        bbox + self
    }
}
