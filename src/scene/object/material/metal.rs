use serde::{Deserialize, Serialize};

use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

#[typetag::serde]
impl ScatterRay for Metal {
    fn scatter_ray(
        &self,
        incoming_ray: &Ray,
        intersection: &Intersection,
    ) -> Option<(Ray, &Color)> {
        let reflect_direction = Vec3::reflect(&incoming_ray.dir, &intersection.normal);

        // generate a random vector with length < 1 to use to displace the reflection vector
        let random_subunit = Vec3::random_subunit_vector();

        // FIXME: we should probably normalize ray direction vectors? because if not, it means
        //   we displacing shorted reflected direction vectors by a greater angle (on average)
        //   than long ones. Maybe that isn't a problem though
        let displaced_reflection = &(self.fuzz * &random_subunit) + &reflect_direction;

        // absorb this ray if the scattered ray points opposite (negative dot product)
        // the normal of the surface
        if Vec3::dot(&displaced_reflection, &intersection.normal) < 0.0 {
            None
        } else {
            Some((
                Ray::new(intersection.point.clone(), displaced_reflection),
                &self.albedo,
            ))
        }
    }
}
