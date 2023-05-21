use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl ScatterRay for Metal {
    fn scatter_ray(&self, incoming_ray: Ray, intersection: Intersection) -> (Ray, &Color) {
        let reflect_direction = Vec3::reflect(&incoming_ray.dir, &intersection.normal);
        let new_ray = Ray::new(intersection.point, reflect_direction);
        (new_ray, &self.albedo)
    }
}
