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
    // TODO: call this `raytrace` or something similar
    // - all it does is find the closest object and report back the object and intersection info
    // - this separates the logic specific to ray tracing from the logic todo with scatering, materials, etc
    //
    // here's another idea!
    // - add `object` as a field to the Intersection struct
    // - then Scene can implement IntersectRay
    // that feels like it would be a good move towards a tree-like
    //   scene structure, yes?
    pub fn color_for_ray(&self, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == 0 {
            return Color::from_rgb_u8(0, 0, 0);
        }

        // TODO: this is the return object
        let mut closest: Option<(Intersection, &Object)> = None;

        // TODO: keep this block
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

        // TODO: this logic goes in render, probably in a function called `color_for_ray`
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
            // TODO: scene need to expose sky so this can be called from render
            None => self.sky.sky_color_for_direction(ray.dir),
        }
    }

    // TODO: just make camera a public field? don't see a need for this right now
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
