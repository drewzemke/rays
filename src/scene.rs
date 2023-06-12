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

        // HACK: do we need to store both of these?
        let mut closest_intersection = None;
        let mut closest_object = None;

        for object in self.objects.iter() {
            let current_intersection = object.geometry.intersect_ray(&ray);

            if let Some(Intersection { t: current_t, .. }) = current_intersection {
                // reject this intersection if its t value is too small or negative
                if current_t < RAY_MIN_T {
                    continue;
                }

                match closest_intersection {
                    // Don't update the closest intersection only if a larger t was found
                    Some(Intersection { t: closest_t, .. }) if current_t > closest_t => {}
                    _ => {
                        closest_intersection = current_intersection;
                        closest_object = Some(object);
                    }
                }
            }
        }

        match closest_intersection {
            Some(ref intersection) => {
                match closest_object
                    .unwrap()
                    .material
                    .scatter_ray(&ray, intersection)
                {
                    Some((scattered_ray, reflection_color)) => {
                        reflection_color * &self.color_for_ray(scattered_ray, bounce_depth - 1)
                    }
                    None => {
                        // This line only gets hit there's a closest intersection but no closest object... feels like a HACK !
                        Color::from_rgb_u8(0, 0, 0)
                    }
                }
            }
            None => self.sky.sky_color_for_direction(ray.dir),
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
