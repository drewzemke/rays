use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    math::{color::Color, ray::Ray, vec3::Vec3},
    scene::object::geometry::Intersection,
};

use super::ScatterRay;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // Uses Schlick approximation to compute the reflectance of this material
    fn reflectance(cos_theta: f32, refractive_ratio: f32) -> f32 {
        let r0 = ((1.0 - refractive_ratio) / (1.0 + refractive_ratio)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

#[typetag::serde]
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

        let normal = (if intersection.is_into_surface {
            1.0
        } else {
            -1.0
        }) * &intersection.normal;

        let cos_theta = -Vec3::dot(&incoming_ray.dir, &normal) / incoming_ray.dir.length();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // this condition checks if total internal reflection is in effect
        let must_reflect = refractive_ratio * sin_theta > 1.0;

        // compute the reflectance of the material, then determine if this ray will
        // be reflected
        let reflectance = Translucent::reflectance(cos_theta, refractive_ratio);
        let mut rng = rand::thread_rng();
        let reflect_ray = must_reflect || (reflectance > rng.gen());

        let new_ray_dir = if reflect_ray {
            Vec3::reflect(&incoming_ray.dir, &normal)
        } else {
            Vec3::refract(&incoming_ray.dir, &normal, refractive_ratio)
        };

        let new_ray = Ray::new(intersection.point.clone(), new_ray_dir);

        // return white as the color
        Some((new_ray, &self.albedo))
    }
}
