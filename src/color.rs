use image::Rgb;

use crate::{math::vec3::Vec3, shaping::clamp};

// for now, color is just a wrapper around Vec3, with the implicit assertion that values are between 0 and 1,
// and that we're using rgb colors
#[derive(Debug, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3 { x: r, y: g, z: b })
    }

    pub fn r(&self) -> f32 {
        self.0.x
    }

    pub fn g(&self) -> f32 {
        self.0.y
    }

    pub fn b(&self) -> f32 {
        self.0.z
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        // TODO: clamp
        Color(Vec3::new(
            clamp(v.x, 0.0, 1.0),
            clamp(v.y, 0.0, 1.0),
            clamp(v.z, 0.0, 1.0),
        ))
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        Rgb([
            (color.0.x * 255.0) as u8,
            (color.0.y * 255.0) as u8,
            (color.0.z * 255.0) as u8,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_access_color() {
        let color = Color::from_rgb(0.2, 0.8, 0.9);
        assert_eq!(color.r(), 0.2);
        assert_eq!(color.g(), 0.8);
        assert_eq!(color.b(), 0.9);
    }

    #[test]
    fn color_from_vec3() {
        let v = Vec3::new(0.1, 0.2, 0.3);
        let c: Color = v.into();
        assert_eq!(c, Color::from_rgb(0.1, 0.2, 0.3));
    }

    #[test]
    fn color_from_vec3_with_clamping() {
        let v = Vec3::new(-0.4, 1.2, 0.3);
        let c: Color = v.into();
        assert_eq!(c, Color::from_rgb(0.0, 1.0, 0.3));
    }
}
