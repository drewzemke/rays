use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    math::{color::Color, ray::Ray},
};

use self::{
    object::{geometry::Intersection, Object},
    sky::Sky,
};

pub mod object;
pub mod sky;

#[derive(Serialize, Deserialize, Builder)]
pub struct Scene {
    camera: Camera,

    #[builder(each = "add_object")]
    objects: Vec<Object>,

    sky: Sky,
}

// the shortest distance a ray can travel before intersections are allowed.
// helps avoid floating points obnoxiousness
const RAY_MIN_T: f32 = 0.0001;

impl Scene {
    // TODO: some of this logic should probably reside in render
    // maybe scene should only handle single ray intersections, not the recursion? idk
    pub fn color_for_ray(&self, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == 0 {
            return Color::from_rgb_u8(0, 0, 0);
        }

        let mut closest: Option<(Intersection, &Object)> = None;

        for object in self.objects.iter() {
            if let Some(intersection) = object.geometry.intersect_ray(&ray) {
                // reject this intersection if its t value is too small or negative
                if intersection.t < RAY_MIN_T {
                    continue;
                }

                match closest {
                    // Update the closest intersection if a larger t was found
                    Some((Intersection { t: closest_t, .. }, _)) if intersection.t > closest_t => {}
                    _ => {
                        closest = Some((intersection, object));
                    }
                };
            }
        }

        match closest {
            Some((ref intersection, object)) => {
                match object.material.scatter_ray(&ray, intersection) {
                    Some((scattered_ray, reflection_color)) => {
                        reflection_color * &self.color_for_ray(scattered_ray, bounce_depth - 1)
                    }
                    // The scattering algorithm decided to absorb the ray, so return black
                    None => Color::from_rgb_u8(0, 0, 0),
                }
            }
            // No intersections, so query the sky for a color
            None => self.sky.sky_color_for_direction(ray.dir),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
