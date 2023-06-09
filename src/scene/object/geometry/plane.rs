use serde::{Deserialize, Serialize};

use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::scene::object::geometry::{IntersectRay, Intersection};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plane {
    basepoint: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(basepoint: Vec3, normal: Vec3) -> Plane {
        Plane { basepoint, normal }
    }
}

#[typetag::serde]
impl IntersectRay for Plane {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let normal_vs_displ = Vec3::dot(&self.normal, &(&ray.origin - &self.basepoint));
        let normal_vs_dir = Vec3::dot(&self.normal, &ray.dir);

        // This condition makes planes two sided.
        // For one sided intersections, replace with this:
        //    normal_vs_displ > 0.0 && normal_vs_dir < 0.0
        let t = -normal_vs_displ / normal_vs_dir;
        if t > 0.0 {
            Some(Intersection {
                point: ray.at(t),
                normal: self.normal.clone(),
                t,
                is_into_surface: normal_vs_displ > 0.0 && normal_vs_dir < 0.0,
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
    fn ray_misses_plane() {
        let ray = Ray::new(Vec3::new(0.0, 1.0, -3.0), Vec3::new(0.0, 0.0, 1.0));
        let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(plane.intersect_ray(&ray), None)
    }

    #[test]
    fn ray_hits_plane() {
        let ray = Ray::new(Vec3::new(0.0, 1.0, 3.0), Vec3::new(0.0, 0.0, -1.0));
        let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(
            plane.intersect_ray(&ray),
            Some(Intersection {
                point: Vec3::new(0.0, 1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                t: 3.0,
                is_into_surface: true
            })
        )
    }
}
