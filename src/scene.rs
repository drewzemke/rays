use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{camera::Camera, math::ray::Ray};

use self::{
    object::{geometry::Intersection, Object},
    sky::Sky,
};

pub mod object;
pub mod sky;

#[derive(Serialize, Deserialize, Builder)]
pub struct Scene {
    pub camera: Camera,

    #[builder(each = "add_object")]
    objects: Vec<Object>,

    pub sky: Sky,
}

// the shortest distance a ray can travel before intersections are allowed.
// helps avoid floating points obnoxiousness
const RAY_MIN_T: f32 = 0.0001;

impl Scene {
    // TODO: something to consider -- it sorta makes sense to name this the same as the method in IntersectRay,
    //   but the return types of those two functions are different. How to reconcile?
    //   Maybe two traits (IntersectRayGeom and IntersectRayObj)?
    pub fn intersect_ray(&self, ray: &Ray) -> Option<(Intersection, &Object)> {
        let mut closest: Option<(Intersection, &Object)> = None;

        for object in self.objects.iter() {
            if let Some(intersection) = object.geometry.intersect_ray(ray) {
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

        closest
    }
}
