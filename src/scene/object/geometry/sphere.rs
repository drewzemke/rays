use crate::math::{ray::Ray, vec3::Vec3};

use crate::scene::object::{IntersectRay, Intersection};

#[derive(Debug)]
pub struct Sphere {
    radius: f32,
    center: Vec3,
}

impl Sphere {
    pub fn new(radius: f32, center: Vec3) -> Sphere {
        Sphere { radius, center }
    }
}

impl IntersectRay for Sphere {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let a = &ray.origin;
        let b = &ray.dir;
        let c = &self.center;
        let r = self.radius;
        let a_min_c = a - c;

        // quadratic equation coefficients
        let q_a = Vec3::dot(b, b);
        let q_b = 2.0 * Vec3::dot(b, &a_min_c);
        let q_c = Vec3::dot(&a_min_c, &a_min_c) - r * r;

        let disc = q_b * q_b - 4.0 * q_a * q_c;
        if disc >= 0.0 {
            // take (what is probably?) the closest intersection
            let t = (-q_b - disc.powf(0.5)) / (2.0 * q_a);

            // HACK: this should be a constraint managed outside of this function
            if t > 0.001 {
                let point = ray.at(t);
                let normal = (&point - c).normalize();
                Some(Intersection { point, normal, t })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_sphere_at_origin() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3::new(0.0, 0.0, 0.0),
        };
        assert_eq!(
            sphere.intersect_ray(&ray),
            Some(Intersection {
                point: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
                t: 2.0
            })
        )
    }

    #[test]
    fn ray_misses_sphere_at_origin() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3::new(0.0, 0.0, 0.0),
        };
        assert_eq!(sphere.intersect_ray(&ray), None)
    }

    #[test]
    fn ray_misses_shifted() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3::new(2.0, 0.0, 0.0),
        };
        assert_eq!(sphere.intersect_ray(&ray), None)
    }
}
