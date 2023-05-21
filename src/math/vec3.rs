use rand::Rng;
use std::{
    f32::consts::PI,
    ops::{Add, Mul, Neg, Sub},
};

// TODO: replace 'f32' with a more generic type?
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

const EPSILON: f32 = 1e-8;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn negative(u: &Vec3) -> Vec3 {
        Vec3::new(-u.x, -u.y, -u.z)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn length(&self) -> f32 {
        Vec3::dot(self, self).powf(0.5)
    }

    pub fn normalize(&self) -> Vec3 {
        (1.0 / self.length()) * self
    }

    // This is based on some not-trivial-but-also-not-the-worst math.
    // You can prove that vectors generated according to this formula
    // are uniformly distributed on the unit sphere.
    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        // generate two numbers in [0,1]
        let s: f32 = rng.gen();
        let t: f32 = rng.gen();

        // transform to be in [0, 2pi] and [-1,1], respectively
        let s1 = 2.0 * PI * s;
        let t1 = 2.0 * t - 1.0;

        // just to DRY this up a bit:
        let sq = (1.0 - t1.powi(2)).powf(0.5);

        let x = s1.cos() * sq;
        let y = s1.sin() * sq;
        let z = t1;

        Vec3::new(x, y, z)
    }

    pub fn is_small(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_access_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn add_vecs() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(&u + &v, Vec3::new(1.0, 6.0, 11.0))
    }

    #[test]
    fn negate_vec() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-&u, Vec3::new(-1.0, -2.0, -3.0))
    }

    #[test]
    fn subtract_vecs() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(&u - &v, Vec3::new(1.0, -2.0, -5.0))
    }

    #[test]
    fn scalar_mult() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let k = -5.0f32;
        assert_eq!(k * &u, Vec3::new(-5.0, -10.0, -15.0))
    }

    #[test]
    fn dot_vecs() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(Vec3::dot(&u, &v), 32.0)
    }

    #[test]
    fn compute_vec_length() {
        let u = Vec3::new(4.0, 0.0, 3.0);
        assert_eq!(u.length(), 5.0)
    }

    #[test]
    fn normalize_vec() {
        let u = Vec3::new(4.0, 0.0, 3.0);
        assert_eq!(u.normalize(), Vec3::new(0.8, 0.0, 0.6))
    }

    #[test]
    fn create_unit_vec() {
        let v = Vec3::random_unit_vector();
        // we can't really verify that this is randomly distributed, unfortunately
        assert!(v.length() - 1.0 < 0.00000001)
        // yay for float comparison
    }
}
