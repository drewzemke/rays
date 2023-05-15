use self::object::sphere::Sphere;

pub mod object;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Sphere>,
}
