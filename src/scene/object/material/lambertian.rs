use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl ScatterRay for Lambertian {
    fn scatter_ray(&self, _incoming_ray: Ray, intersection: Intersection) -> (Ray, &Color) {
        let random_unit = Vec3::random_unit_vector();
        let new_ray = Ray::new(intersection.point, &intersection.normal + &random_unit);
        (new_ray, &self.albedo)
    }
}
