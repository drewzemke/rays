use super::{vec3::Vec3, SphereAtOrigin};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub point: Vec3,
    // assumed to have unit length
    pub normal: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }

    pub fn intersect_sphere(&self, sphere: &SphereAtOrigin) -> Option<Intersection> {
        let a = self.origin;
        let b = self.dir;
        let c = Vec3::new(0.0, 0.0, 0.0);
        let r = sphere.radius;
        let a_min_c = a - c;

        // quadratic equation coefficients
        let q_a = Vec3::dot(&b, &b);
        let q_b = 2.0 * Vec3::dot(&b, &a_min_c);
        let q_c = Vec3::dot(&a_min_c, &a_min_c) - r * r;

        let disc = q_b * q_b - 4.0 * q_a * q_c;
        if disc >= 0.0 {
            // take (what is probably?) the closest intersection
            let t = (-q_b - disc.powf(0.5)) / (2.0 * q_a);
            let point = self.at(t);
            let normal = point.normalize();
            Some(Intersection { point, normal })
        } else {
            None
        }
    }
}
