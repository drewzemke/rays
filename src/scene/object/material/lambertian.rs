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
    fn scatter_ray(&self, _incoming_ray: &Ray, intersection: &Intersection) -> (Ray, &Color) {
        let random_unit = Vec3::random_unit_vector();

        let mut scatter_dir = &intersection.normal + &random_unit;

        // reject scattered vectors that are too close to zero
        if scatter_dir.is_small() {
            scatter_dir = intersection.normal.clone();
        }

        let new_ray = Ray::new(intersection.point.clone(), scatter_dir);
        (new_ray, &self.albedo)
    }
}
