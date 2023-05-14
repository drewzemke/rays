pub mod ray;
pub mod vec3;

// TODO: move this somewhere else
pub struct SphereAtOrigin {
    radius: f32,
}

impl SphereAtOrigin {
    pub fn new(radius: f32) -> SphereAtOrigin {
        SphereAtOrigin { radius }
    }
}

#[cfg(test)]
mod sphere_intersect_tests {
    use super::{
        ray::{Intersection, Ray},
        vec3::Vec3,
        *,
    };

    #[test]
    fn ray_hits_sphere() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let sphere = SphereAtOrigin { radius: 1.0 };
        assert_eq!(
            ray.intersect_sphere(&sphere),
            Some(Intersection {
                point: Vec3::new(0.0, 0.0, -1.0),
                normal: Vec3::new(0.0, 0.0, -1.0)
            })
        )
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let sphere = SphereAtOrigin { radius: 1.0 };
        assert_eq!(ray.intersect_sphere(&sphere), None)
    }
}
