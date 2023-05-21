use self::{geometry::IntersectRay, material::ScatterRay};

pub mod geometry;
pub mod material;

pub struct Object<'a> {
    pub geometry: &'a dyn IntersectRay,
    pub material: &'a dyn ScatterRay,
}
