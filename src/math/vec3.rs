use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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
        (1.0 / self.length()) * *self
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
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
        assert_eq!(u + v, Vec3::new(1.0, 6.0, 11.0))
    }

    #[test]
    fn negate_vec() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(-u, Vec3::new(-1.0, -2.0, -3.0))
    }

    #[test]
    fn subtract_vecs() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(u - v, Vec3::new(1.0, -2.0, -5.0))
    }

    #[test]
    fn scalar_mult() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let k = -5.0f32;
        assert_eq!(k * u, Vec3::new(-5.0, -10.0, -15.0))
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
}
