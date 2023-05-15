use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub point: Vec3,
    // assumed to have unit length
    pub normal: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}

pub trait IntersectRay {
    fn intersect_ray(&self, ray: &Ray) -> Option<Intersection>;
}
