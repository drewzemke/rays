use crate::math::{color::Color, ray::Ray, shaping::lerp, vec3::Vec3};

use self::object::{geometry::Intersection, Object};

pub mod object;

pub struct Scene<'a> {
    objects: Vec<&'a Object<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(objects: Vec<&'a Object>) -> Scene<'a> {
        Scene { objects }
    }

    // TODO: some of this logic should probably reside in render
    // maybe scene should only handle single ray intersections, not the recursion? idk
    pub fn color_for_ray(&self, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == 0 {
            return Color::from_rgb_u8(0, 0, 0);
        }

        // HACK: do we need to store both of these?
        let mut closest_intersection = None;
        let mut closest_object = None;

        for object in self.objects.as_slice().iter() {
            let current_intersection = object.geometry.intersect_ray(&ray);

            if let Some(Intersection { t: current_t, .. }) = current_intersection {
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
            Some(intersection) => {
                let (scattered_ray, reflection_color) = closest_object
                    .unwrap()
                    .material
                    .scatter_ray(ray, intersection);

                reflection_color * &self.color_for_ray(scattered_ray, bounce_depth - 1)
            }
            None => self.sky_color_for_direction(ray.dir),
        }
    }

    fn sky_color_for_direction(&self, dir: Vec3) -> Color {
        // TODO: make these params
        let nadir_color = Color::from_rgb_f32(1.0, 1.0, 1.0);
        let zenith_color = Color::from_rgb_f32(0.5, 0.7, 1.0);

        let t = 0.5 * (dir.y + 1.0);
        lerp(t, &nadir_color, &zenith_color)
    }
}
