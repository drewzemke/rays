use serde::{Deserialize, Serialize};

use self::{geometry::IntersectRay, material::ScatterRay};

pub mod geometry;
pub mod material;

type Geometry = Box<dyn IntersectRay>;
type Material = Box<dyn ScatterRay>;

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}
