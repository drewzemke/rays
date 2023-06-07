use serde::{Deserialize, Serialize};

use self::{geometry::IntersectRay, material::ScatterRay};

pub mod geometry;
pub mod material;

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub geometry: Box<dyn IntersectRay>,
    pub material: Box<dyn ScatterRay>,
}
