use serde::{Deserialize, Serialize};

use crate::math::{ray::Ray, vec3::Vec3};

use crate::scene::object::geometry::{IntersectRay, Intersection};

use super::NormalOrientation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
    radius: f32,
    center: Vec3,
    orientation: NormalOrientation,
}

impl Sphere {
    pub fn new(radius: f32, center: Vec3) -> Sphere {
        Sphere {
            radius,
            center,
            orientation: NormalOrientation::Outward,
        }
    }
    pub fn flip_orientation(&mut self) {
        self.orientation = match self.orientation {
            NormalOrientation::Outward => NormalOrientation::Inward,
            NormalOrientation::Inward => NormalOrientation::Outward,
        };
    }
}

#[typetag::serde]
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
            // if ||a-c|| ~= r, the ray originated inside the sphere,
            // and if the (a-c).b is negative then the ray is pointing into the sphere,
            // so the larger of the two solutions to the quadratic equation is the
            // only positive solution
            let t;
            let mut into_surface;

            // HACK: subtracting the 0.0001 dodges some floating point oddities
            //   it'd be better to have a more global / general solution
            if q_c.abs() < 0.0001 && q_b < 0.0 {
                t = (-q_b + disc.powf(0.5)) / (2.0 * q_a);
                into_surface = false;
            } else {
                t = (-q_b - disc.powf(0.5)) / (2.0 * q_a);
                into_surface = true;
            }

            // otherwise, the smaller of the two solutions is the first one along the ray

            let point = ray.at(t);
            let normal_sign = match self.orientation {
                NormalOrientation::Outward => 1.0,
                NormalOrientation::Inward => {
                    // HACK: oh god is this a hack
                    // really need to find a better way to sort out how the orientation and `into_surface` interact
                    into_surface = !into_surface;
                    -1.0
                }
            };
            let normal = normal_sign * &(&point - c).normalize();
            Some(Intersection {
                point,
                normal,
                t,
                is_into_surface: into_surface,
            })
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
            orientation: NormalOrientation::Outward,
        };
        assert_eq!(
            sphere.intersect_ray(&ray),
            Some(Intersection {
                point: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0),
                t: 2.0,
                is_into_surface: true
            })
        )
    }

    #[test]
    fn ray_misses_sphere_at_origin() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3::new(0.0, 0.0, 0.0),
            orientation: NormalOrientation::Outward,
        };
        assert_eq!(sphere.intersect_ray(&ray), None)
    }

    #[test]
    fn ray_misses_shifted() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            radius: 1.0,
            center: Vec3::new(2.0, 0.0, 0.0),
            orientation: NormalOrientation::Outward,
        };
        assert_eq!(sphere.intersect_ray(&ray), None)
    }
}
