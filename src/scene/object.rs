use crate::math::{ray::Ray, vec3::Vec3};

pub mod geometry;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    // where (in world space) the intersection occurs
    pub point: Vec3,

    // assumed to have unit length
    pub normal: Vec3,

    // the value of the parameter at the intersection
    pub t: f32,
}

pub trait IntersectRay {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
}
