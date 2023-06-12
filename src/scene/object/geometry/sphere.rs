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
        // the displacement vector from the sphere center to the origin of the ray
        let disp = (&ray.origin) - (&self.center);

        // quadratic equation coefficients
        let q_a = Vec3::dot(&ray.dir, &ray.dir);
        let q_b = 2.0 * Vec3::dot(&ray.dir, &disp);
        let q_c = Vec3::dot(&disp, &disp) - self.radius.powi(2);

        let disc = q_b * q_b - 4.0 * q_a * q_c;
        if disc >= 0.0 {
            // NOTE: if |disp| ~= r, the ray originated on the the sphere itself,
            // and if (disp).dir is negative then the ray is pointing into the sphere,
            // HACK: subtracting the 0.0001 dodges some floating point oddities
            //   it'd be better to have a more global / general solution
            // FIXME: we're not handling the case where the ray starts properly inside the sphere
            let ray_starts_in_sphere = q_c.abs() < 0.0001 && q_b < 0.0;

            let is_into_surface = match self.orientation {
                NormalOrientation::Outward => !ray_starts_in_sphere,
                NormalOrientation::Inward => ray_starts_in_sphere,
            };

            let t = if ray_starts_in_sphere {
                // the first solution will be negative or close to zero, so the larger of the two
                // solutions to the quadratic equation is the only positive solution
                (-q_b + disc.powf(0.5)) / (2.0 * q_a)
            } else {
                // the ray passes through the sphere twice, so the smaller of the two solutions
                // is the first one along the ray
                (-q_b - disc.powf(0.5)) / (2.0 * q_a)
            };

            let point = ray.at(t);
            let normal = match self.orientation {
                NormalOrientation::Outward => (&point - (&self.center)).normalize(),
                NormalOrientation::Inward => ((&self.center) - &point).normalize(),
            };

            Some(Intersection {
                point,
                normal,
                t,
                is_into_surface,
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
