#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    // Negation
    pub fn neg(&self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    // Indexing (immutable)
    pub fn get(&self, i: usize) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }

    // Indexing (mutable)
    pub fn get_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }

    // Compound assignment: +=
    pub fn add_assign(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    // Compound assignment: *= scalar
    pub fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }

    // Compound assignment: /= scalar
    pub fn div_assign(&mut self, t: f64) {
        let inv = 1.0 / t;
        self.mul_assign(inv);
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        *v - 2.0 * Vector3::dot(v, n) * (*n)
    }

    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(Vector3::dot(&(-*uv), n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * (*n));
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*n);
        r_out_perp + r_out_parallel
    }
}

// Operator overloads and utility functions
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        Vector3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        Vector3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        (1.0 / t) * self
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, t: f64) {
        let inv = 1.0 / t;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl Index<usize> for Vector3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

// Utility functions
impl Vector3 {
    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(&self) -> Vector3 {
        *self / self.length()
    }

    pub fn random() -> Vector3 {
        Vector3 {
            x: random_range(0f64..1f64),
            y: random_range(0f64..1f64),
            z: random_range(0f64..1f64),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vector3 {
        Vector3 {
            x: random_range(min..max),
            y: random_range(min..max),
            z: random_range(min..max),
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        loop {
            let p = Vector3::random_range(-1., 1.);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let on_unit_sphere = Vector3::random_unit_vector();
        if Vector3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let p = Vector3::new(random_range(-1f64..1f64), random_range(-1f64..1f64), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

// Alias for geometric clarity
pub type Point3 = Vector3;

// Display implementation
use rand::random_range;
use std::fmt;

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
