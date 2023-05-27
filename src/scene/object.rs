use self::{geometry::IntersectRay, material::ScatterRay};

pub mod geometry;
pub mod material;

pub struct Object {
    pub geometry: Box<dyn IntersectRay>,
    pub material: Box<dyn ScatterRay>,
}
