use crate::math::{color::Color, ray::Ray};

use super::Intersection;

pub mod lambertian;

pub trait ScatterRay {
    fn scatter_ray(&self, incoming_ray: Ray, intersection: Intersection) -> (Ray, &Color);
}
