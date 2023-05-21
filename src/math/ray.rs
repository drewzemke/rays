use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        &self.origin + &(t * &self.dir)
    }
}
