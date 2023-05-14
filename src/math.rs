use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn negative(u: &Vec3) -> Vec3 {
        Vec3::new(-u.x, -u.y, -u.z)
    }

    pub fn scalar_mult(u: &Vec3, k: f32) -> Vec3 {
        Vec3::new(k * u.x, k * u.y, k * u.z)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3::new(self.x, self.y, self.z)
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

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(test)]
mod vec3_tests {
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
        assert_eq!(u.add(&v), Vec3::new(1.0, 6.0, 11.0))
    }

    #[test]
    fn add_vecs_infix() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(&u + &v, Vec3::new(1.0, 6.0, 11.0))
    }

    #[test]
    fn negate_vec() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::negative(&u), Vec3::new(-1.0, -2.0, -3.0))
    }

    #[test]
    fn subtract_vecs() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(u.sub(&v), Vec3::new(1.0, -2.0, -5.0))
    }

    #[test]
    fn subtract_vecs_infix() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        assert_eq!(&u - &v, Vec3::new(1.0, -2.0, -5.0))
    }

    #[test]
    fn scalar_mult() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let k = -5.0;
        assert_eq!(Vec3::scalar_mult(&u, k), Vec3::new(-5.0, -10.0, -15.0))
    }

    #[test]
    fn scalar_mult_infix() {
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
}

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        &self.origin + &Vec3::scalar_mult(&self.dir, t)
    }

    pub fn intersect_sphere(&self, sphere: &SphereAtOrigin) -> bool {
        let a = &self.origin;
        let b = &self.dir;
        let c = Vec3::new(0.0, 0.0, 0.0);
        let r = sphere.radius;
        let a_min_c = a - &c;

        // quadratic equation coefficients
        let q_a = Vec3::dot(b, b);
        let q_b = 2.0 * Vec3::dot(b, &a_min_c);
        let q_c = Vec3::dot(&a_min_c, &a_min_c) - r * r;

        q_b * q_b - 4.0 * q_a * q_c >= 0.0
    }
}

pub struct SphereAtOrigin {
    radius: f32,
}

impl SphereAtOrigin {
    pub fn new(radius: f32) -> SphereAtOrigin {
        SphereAtOrigin { radius }
    }
}

#[cfg(test)]
mod sphere_intersect_tests {
    use super::*;

    #[test]
    fn ray_hits_sphere() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let sphere = SphereAtOrigin { radius: 1.0 };
        assert!(ray.intersect_sphere(&sphere))
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
        };
        let sphere = SphereAtOrigin { radius: 1.0 };
        assert!(!ray.intersect_sphere(&sphere))
    }
}
