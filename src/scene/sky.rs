use serde::{Deserialize, Serialize};

use crate::math::{color::Color, shaping::lerp, vec3::Vec3};

#[derive(Serialize, Deserialize)]
pub struct Sky {
    nadir: Color,
    zenith: Color,
}

impl Sky {
    pub fn new(nadir: Color, zenith: Color) -> Sky {
        Sky { nadir, zenith }
    }

    pub fn sky_color_for_direction(&self, dir: &Vec3) -> Color {
        let t = 0.5 * (dir.y + 1.0);
        lerp(t, &self.nadir, &self.zenith)
    }
}
