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

        let cos_theta =
            Vec3::dot(&incoming_ray.dir, &intersection.normal) / incoming_ray.dir.length();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // this condition checks if total internal reflection is in effect
        let must_reflect = refractive_ratio * sin_theta > 1.0;

        let normal = (if intersection.is_into_surface {
            1.0
        } else {
            -1.0
        }) * &intersection.normal;

        let new_ray_dir = if must_reflect {
            Vec3::reflect(&incoming_ray.dir, &normal)
        } else {
            Vec3::refract(&incoming_ray.dir, &normal, refractive_ratio)
        };

        let new_ray = Ray::new(intersection.point.clone(), new_ray_dir);

        // return white as the color
        Some((new_ray, &self.albedo))
    }
}
