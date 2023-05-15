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
    // objects: Vec<Sphere>,
}

impl Scene {
    pub fn color_for_ray(&self, ray: Ray) -> Color {
        // let sphere = &self.objects[0];

        let sphere = Sphere::new(1.0, Vec3::new(1.0, 0.0, 0.0));

        // sky color
        let zenith_col = Color::from_rgb(1.0, 1.0, 1.0);
        let nadir_col = Color::from_rgb(0.5, 0.7, 1.0);

        match sphere.intersect_ray(&ray) {
            Some(Intersection { point: _, normal }) => {
                let mapped_normal = 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
                mapped_normal.into()
            }
            None => {
                // TODO: remap!!
                let t = 0.5 * (ray.dir.y + 1.0);
                lerp(t, zenith_col, nadir_col)
            }
        }
    }
}
