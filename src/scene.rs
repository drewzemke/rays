use crate::math::{
    color::Color,
    ray::{IntersectRay, Intersection, Ray},
    shaping::lerp,
    vec3::Vec3,
};

pub mod object;

pub struct Scene {
    objects: Vec<Box<dyn IntersectRay>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn IntersectRay>>) -> Scene {
        Scene { objects }
    }

    // TODO: some of this logic should probably reside in render
    // maybe scene should only handle single ray intersections, not the recursion? idk
    pub fn color_for_ray(&self, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == 0 {
            return Color::from_rgb(0.0, 0.0, 0.0);
        }

        let mut closest_intersection = None;

        for object in self.objects.as_slice().iter() {
            let current_intersection = object.intersect_ray(&ray);

            // FIXME: this looks like butt
            if let Some(Intersection {
                point: _,
                normal: _,
                t,
            }) = current_intersection
            {
                match closest_intersection {
                    Some(Intersection {
                        point: _,
                        normal: _,
                        t: closest_t,
                    }) => {
                        if t < closest_t {
                            closest_intersection = current_intersection;
                        }
                    }
                    None => {
                        closest_intersection = current_intersection;
                    }
                }
            }
        }

        match closest_intersection {
            Some(intersection) => {
                // TODO: modulate by object color
                // generate a diffuse vector, and run it back
                let random_unit = Vec3::random_unit_vector();
                let new_ray = Ray::new(intersection.point, &intersection.normal + &random_unit);

                0.5 * &self.color_for_ray(new_ray, bounce_depth - 1)
            }
            None => self.sky_color_for_direction(ray.dir),
        }
    }

    fn sky_color_for_direction(&self, dir: Vec3) -> Color {
        // TODO: make these params
        let nadir_color = Color::from_rgb(1.0, 1.0, 1.0);
        let zenith_color = Color::from_rgb(0.5, 0.7, 1.0);

        let t = 0.5 * (dir.y + 1.0);
        lerp(t, &nadir_color, &zenith_color)
    }
}
