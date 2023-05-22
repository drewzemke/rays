use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl ScatterRay for Metal {
    fn scatter_ray(&self, incoming_ray: Ray, intersection: Intersection) -> (Ray, &Color) {
        let reflect_direction = Vec3::reflect(&incoming_ray.dir, &intersection.normal);

        // generate a random vector with length < 1 to use to displace the reflection vector
        let random_subunit = Vec3::random_subunit_vector();

        // FIXME: we should probably normalize ray direction vectors? because if not, it means
        //   we displacing shorted reflected direction vectors by a greater angle (on average)
        //   than long ones. Maybe that isn't a problem though
        let displaced_reflection = &(self.fuzz * &random_subunit) + &reflect_direction;

        let new_ray = Ray::new(intersection.point, displaced_reflection);
        (new_ray, &self.albedo)
    }
}
