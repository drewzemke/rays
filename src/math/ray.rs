use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

#[derive(Debug, PartialEq)]
pub struct Intersection {
    // where (in world space) the intersection occurs
    pub point: Vec3,

    // assumed to have unit length
    pub normal: Vec3,

    // the value of the parameter at the intersection
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}

// Move to object.rs?
pub trait IntersectRay {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
}
