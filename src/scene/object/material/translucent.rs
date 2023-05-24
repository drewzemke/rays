use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

pub struct Translucent {
    albedo: Color,
    refractive_index: f32,
}

impl Translucent {
    pub fn new(refractive_index: f32) -> Translucent {
        Translucent {
            albedo: Color::from_rgb_u8(255, 255, 255),
            refractive_index,
        }
    }
}

impl ScatterRay for Translucent {
    fn scatter_ray(
        &self,
        incoming_ray: &Ray,
        intersection: &Intersection,
    ) -> Option<(Ray, &Color)> {
        let refracted_dir = if intersection.is_into_surface {
            Vec3::refract(
                &incoming_ray.dir,
                &intersection.normal,
                1.0,
                self.refractive_index,
            )
        } else {
            Vec3::refract(
                &incoming_ray.dir,
                &(-&intersection.normal),
                self.refractive_index,
                1.0,
            )
        };

        let new_ray = Ray::new(intersection.point.clone(), refracted_dir);

        // return white as the color
        Some((new_ray, &self.albedo))
    }
}
