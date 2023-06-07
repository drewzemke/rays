use super::geometry::Intersection;
use crate::math::{color::Color, ray::Ray};

pub mod lambertian;
pub mod metal;
pub mod translucent;

#[typetag::serde]
pub trait ScatterRay {
    // QUESTION: Should this trait know about Intersection? or should it take intersection info as input directly?
    fn scatter_ray(&self, incoming_ray: &Ray, intersection: &Intersection)
        -> Option<(Ray, &Color)>;
}
