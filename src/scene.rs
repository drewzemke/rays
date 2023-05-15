use self::object::sphere::Sphere;
use crate::math::{
    color::Color,
    ray::{IntersectRay, Intersection, Ray},
    shaping::lerp,
    vec3::Vec3,
};

pub mod object;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Sphere>,
}

impl Scene {
    pub fn new(objects: Vec<Sphere>) -> Scene {
        Scene { objects }
    }

    pub fn color_for_ray(&self, ray: Ray) -> Color {
        let mut intersection_color = None;
        let mut intersection_t = f32::INFINITY;

        // sky color -- extract to somewhere else
        let nadir_color = Color::from_rgb(1.0, 1.0, 1.0);
        let zenith_color = Color::from_rgb(0.5, 0.7, 1.0);
        for sphere in self.objects.as_slice().iter() {
            if let Some(Intersection {
                point: _,
                normal,
                t,
            }) = sphere.intersect_ray(&ray)
            {
                if t < intersection_t {
                    intersection_t = t;
                    // TODO: use object color
                    let mapped_normal = 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
                    intersection_color = Some(mapped_normal.into());
                }
            }
        }

        if let Some(color) = intersection_color {
            color
        } else {
            let t = 0.5 * (ray.dir.y + 1.0);
            lerp(t, nadir_color, zenith_color)
        }
    }
}
