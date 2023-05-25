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
        let refractive_ratio = if intersection.is_into_surface {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let normal_sign = if intersection.is_into_surface {
            1.0
        } else {
            -1.0
        };

        let refracted_dir = Vec3::refract(
            &incoming_ray.dir,
            &(normal_sign * &intersection.normal),
            refractive_ratio,
        );

        let new_ray = Ray::new(intersection.point.clone(), refracted_dir);

        // return white as the color
        Some((new_ray, &self.albedo))
    }
}
