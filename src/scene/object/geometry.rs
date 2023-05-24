use crate::math::{ray::Ray, vec3::Vec3};

pub mod plane;
pub mod sphere;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    // where (in world space) the intersection occurs
    pub point: Vec3,

    // assumed to have unit length and point from the inside out (for closed surfaces)
    pub normal: Vec3,

    // the value of the parameter at the intersection
    pub t: f32,

    // whether the ray is passing through the surface opposite the direction of the normal
    // at the intersection point
    // (it's easier to compute and store this during intersection computation, rather than later)
    pub is_into_surface: bool,
}

pub trait IntersectRay {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
}
