use crate::vector::{Vector3, Point3};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Rotation {
    pub angle_radians: f64,
    pub cos_theta: f64,
    pub sin_theta: f64,
    pub axis: RotationAxis,
}

#[derive(Clone, Copy, Debug)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

impl Rotation {
    pub fn rotate_x(angle_degrees: f64) -> Self {
        let angle_radians = angle_degrees * PI / 180.0;
        Self {
            angle_radians,
            cos_theta: angle_radians.cos(),
            sin_theta: angle_radians.sin(),
            axis: RotationAxis::X,
        }
    }

    pub fn rotate_y(angle_degrees: f64) -> Self {
        let angle_radians = angle_degrees * PI / 180.0;
        Self {
            angle_radians,
            cos_theta: angle_radians.cos(),
            sin_theta: angle_radians.sin(),
            axis: RotationAxis::Y,
        }
    }

    pub fn rotate_z(angle_degrees: f64) -> Self {
        let angle_radians = angle_degrees * PI / 180.0;
        Self {
            angle_radians,
            cos_theta: angle_radians.cos(),
            sin_theta: angle_radians.sin(),
            axis: RotationAxis::Z,
        }
    }

    pub fn transform_point(&self, point: &Point3) -> Point3 {
        match self.axis {
            RotationAxis::X => Point3::new(
                point.x(),
                self.cos_theta * point.y() - self.sin_theta * point.z(),
                self.sin_theta * point.y() + self.cos_theta * point.z(),
            ),
            RotationAxis::Y => Point3::new(
                self.cos_theta * point.x() + self.sin_theta * point.z(),
                point.y(),
                -self.sin_theta * point.x() + self.cos_theta * point.z(),
            ),
            RotationAxis::Z => Point3::new(
                self.cos_theta * point.x() - self.sin_theta * point.y(),
                self.sin_theta * point.x() + self.cos_theta * point.y(),
                point.z(),
            ),
        }
    }

    pub fn transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.transform_point(vector)
    }

    pub fn inverse_transform_point(&self, point: &Point3) -> Point3 {
        match self.axis {
            RotationAxis::X => Point3::new(
                point.x(),
                self.cos_theta * point.y() + self.sin_theta * point.z(),
                -self.sin_theta * point.y() + self.cos_theta * point.z(),
            ),
            RotationAxis::Y => Point3::new(
                self.cos_theta * point.x() - self.sin_theta * point.z(),
                point.y(),
                self.sin_theta * point.x() + self.cos_theta * point.z(),
            ),
            RotationAxis::Z => Point3::new(
                self.cos_theta * point.x() + self.sin_theta * point.y(),
                -self.sin_theta * point.x() + self.cos_theta * point.y(),
                point.z(),
            ),
        }
    }

    pub fn inverse_transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.inverse_transform_point(vector)
    }
}

#[derive(Clone, Debug)]
pub struct CompositeRotation {
    pub rotations: Vec<Rotation>,
}

impl CompositeRotation {
    pub fn new() -> Self {
        Self {
            rotations: Vec::new(),
        }
    }

    pub fn rotate_x(mut self, angle_degrees: f64) -> Self {
        self.rotations.push(Rotation::rotate_x(angle_degrees));
        self
    }

    pub fn rotate_y(mut self, angle_degrees: f64) -> Self {
        self.rotations.push(Rotation::rotate_y(angle_degrees));
        self
    }

    pub fn rotate_z(mut self, angle_degrees: f64) -> Self {
        self.rotations.push(Rotation::rotate_z(angle_degrees));
        self
    }

    pub fn transform_point(&self, point: &Point3) -> Point3 {
        self.rotations.iter().fold(*point, |p, rotation| {
            rotation.transform_point(&p)
        })
    }

    pub fn transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.rotations.iter().fold(*vector, |v, rotation| {
            rotation.transform_vector(&v)
        })
    }

    pub fn inverse_transform_point(&self, point: &Point3) -> Point3 {
        self.rotations.iter().rev().fold(*point, |p, rotation| {
            rotation.inverse_transform_point(&p)
        })
    }

    pub fn inverse_transform_vector(&self, vector: &Vector3) -> Vector3 {
        self.rotations.iter().rev().fold(*vector, |v, rotation| {
            rotation.inverse_transform_vector(&v)
        })
    }
}
