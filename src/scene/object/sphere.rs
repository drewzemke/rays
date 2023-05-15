use crate::math::{
    ray::{IntersectRay, Intersection, Ray},
    vec3::Vec3,
};

#[derive(Debug)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius }
    }
}

impl IntersectRay for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.origin;
        let b = ray.dir;
        let c = Vec3::new(0.0, 0.0, 0.0);
        let r = self.radius;
        let a_min_c = a - c;

        // quadratic equation coefficients
        let q_a = Vec3::dot(&b, &b);
        let q_b = 2.0 * Vec3::dot(&b, &a_min_c);
        let q_c = Vec3::dot(&a_min_c, &a_min_c) - r * r;

        let disc = q_b * q_b - 4.0 * q_a * q_c;
        if disc >= 0.0 {
            // take (what is probably?) the closest intersection
            let t = (-q_b - disc.powf(0.5)) / (2.0 * q_a);
            let point = ray.at(t);
            let normal = point.normalize();
            Some(Intersection { point, normal })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod sphere_intersect_tests {

    use super::*;

    #[test]
    fn ray_hits_sphere() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere { radius: 1.0 };
        assert_eq!(
            sphere.intersect_ray(&ray),
            Some(Intersection {
                point: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0)
            })
        )
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let sphere = Sphere { radius: 1.0 };
        assert_eq!(sphere.intersect_ray(&ray), None)
    }
}
