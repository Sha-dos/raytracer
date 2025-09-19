use rand::random_range;
use crate::vector::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_floats: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_floats = [0.; POINT_COUNT];
        
        for i in 0..POINT_COUNT {
            rand_floats[i] = random_range(-1f64..1f64);
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        
        Self {
            rand_floats,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        
        let mut c = [[[0.; 2]; 2]; 2];
        
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = (self.perm_x[((i + di as i32) & 255) as usize] +
                               self.perm_y[((j + dj as i32) & 255) as usize] +
                               self.perm_z[((k + dk as i32) & 255) as usize]) & 255;
                    c[di][dj][dk] = self.rand_floats[idx];
                }
            }
        }
        
        Self::trilinear_interp(&c, u, v, w)
    }
    
    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        
        let mut accum = 0.;
        
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_u = if i == 1 { uu } else { 1. - uu };
                    let weight_v = if j == 1 { vv } else { 1. - vv };
                    let weight_w = if k == 1 { ww } else { 1. - ww };
                    accum += weight_u * weight_v * weight_w * c[i][j][k];
                }
            }
        }
        
        accum
    }
    
    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i;
        }
        
        Self::permute(&mut p, POINT_COUNT);
        p
    }
    
    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = random_range(0..i + 1);
            p.swap(i, target);
        }
    }
}