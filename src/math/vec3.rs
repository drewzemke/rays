use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::PI,
    ops::{Add, Mul, Neg, Sub},
};

// TODO: replace 'f32' with a more generic type?
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn length(&self) -> f32 {
        Vec3::dot(self, self).powf(0.5)
    }

    pub fn normalize(&self) -> Vec3 {
        (1.0 / self.length()) * self
    }

    pub fn lin_comb(items: Vec<(f32, &Vec3)>) -> Vec3 {
        items
            .iter()
            .fold(Vec3::new(0.0, 0.0, 0.0), |sum, (coeff, vec)| {
                &sum + &(*coeff * *vec)
            })
    }

    // This is based on some not-trivial-but-also-not-the-worst math.
    // You can prove that vectors generated according to this formula
    // are uniformly distributed on the unit sphere.
    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        // generate two numbers in [0,1)
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

    pub fn random_subunit_vector() -> Vec3 {
        // take a random unit vector and scale it down by a random amount
        let mut rng = rand::thread_rng();
        let unit_vec = Vec3::random_unit_vector();

        // generate a number in [0,1)
        let r: f32 = rng.gen();

        // scaling by r yields a non-uniform distribution, since vectors
        // close to the center of the unit ball are more likely.
        // scaling by the cube root of r gives the correct distribution
        r.powf(1.0 / 3.0) * &unit_vec

        // FIXME: it's probably fine to replace the exponent with 0.333333333, right?
    }

    pub fn is_small(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    // named `unit_normal` because it should be unit length :)
    pub fn reflect(incident: &Vec3, unit_normal: &Vec3) -> Vec3 {
        incident - &(2.0 * Vec3::dot(incident, unit_normal) * unit_normal)
    }

    // the refractive index is n_out/n_in, and we assume that the normal is
    // oriented from the volume with refractive index n_out towards the volume with index n_in
    pub fn refract(incident: &Vec3, normal: &Vec3, refractive_index: f32) -> Vec3 {
        let unit_incident = incident.normalize();
        let dot = Vec3::dot(&unit_incident, normal);

        // parallel and perpendicular components to the surface
        let refracted_parallel = refractive_index * &(&unit_incident + &(-dot * normal));
        let refracted_perp = -(1.0 - refracted_parallel.length().powi(2)).sqrt() * normal;
        &refracted_parallel + &refracted_perp
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
    fn cross_vec() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::cross(&i, &j), k);
        assert_eq!(Vec3::cross(&j, &k), i);
        assert_eq!(Vec3::cross(&k, &i), j);
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
        // we can't easily verify that this is uniformly distributed, unfortunately
        assert!(v.length() - 1.0 < 0.00000001)
        // yay for float comparison
    }

    #[test]
    fn create_subunit_vec() {
        let v = Vec3::random_subunit_vector();
        // again, it'd be really obnixous to check that this is uniformly distributed
        assert!(v.length() < 1.0)
    }

    #[test]
    fn reflect_vector() {
        let n = Vec3::new(0.0, 0.0, 1.0);
        let v = Vec3::new(3.0, 2.0, -1.0);
        assert_eq!(Vec3::reflect(&v, &n), Vec3::new(3.0, 2.0, 1.0));
    }

    #[test]
    fn reflected_vector_dot_product_invariant() {
        let n = Vec3::random_unit_vector();
        let v1 = Vec3::random_unit_vector();
        let v2 = Vec3::reflect(&v1, &n);
        // (n . v1) should equal (- n . v2)
        assert!(Vec3::dot(&n, &v1) + Vec3::dot(&n, &v2) < 1e-6);
    }

    #[test]
    fn refracted_vector_snells_law() {
        let n = Vec3::random_unit_vector();
        let v1 = Vec3::random_unit_vector();

        let n_out = 1.0;
        let n_in = 1.3;
        let v2 = Vec3::refract(&v1, &n, n_out / n_in);

        // sin = sqrt(1-cos^2), and cos is the dot product
        let sin_out = (1.0 - Vec3::dot(&(-&v1), &n).powi(2)).sqrt();
        let sin_in = (1.0 - Vec3::dot(&v2, &(-&n)).powi(2)).sqrt();

        // snell's law
        assert!(n_in * sin_in - n_out * sin_out < 1e-6)
    }

    #[test]
    fn empty_lin_comb_is_zero() {
        assert_eq!(Vec3::lin_comb(vec![]), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn compute_linear_combination() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(0.0, 4.0, 8.0);
        let w = Vec3::new(2.0, -4.0, 0.0);
        let result = Vec3::lin_comb(vec![(1.0, &u), (-2.0, &v), (3.0, &w)]);
        assert_eq!(result, Vec3::new(7.0, -18.0, -13.0));
    }
}
