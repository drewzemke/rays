use std::ops::{Add, Mul};

use image::{ImageBuffer, Rgb};

use crate::{math::vec3::Vec3, shaping::clamp::clamp};

// for now, color is just a wrapper around Vec3, with the implicit assertion that values are between 0 and 1,
// and that we're using rgb colors
#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn from_clamped(v: Vec3) -> Self {
        Color(Vec3::new(
            clamp(v.x, 0.0, 1.0),
            clamp(v.y, 0.0, 1.0),
            clamp(v.z, 0.0, 1.0),
        ))
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from(self.0 + rhs.0)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::from(self * rhs.0)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color(v)
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
        let c: Color = Color::from_clamped(v);
        assert_eq!(c, Color::from_rgb(0.0, 1.0, 0.3));
    }
}

// put this somewhere else eventually
// also, find a better name
pub struct ColorMatrix(Vec<Vec<Color>>);

impl ColorMatrix {
    pub fn new(width: usize, height: usize) -> ColorMatrix {
        let default_color = Color::from_rgb(0.0, 0.0, 0.0);
        let row = vec![default_color; width];
        ColorMatrix(vec![row; height])
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    // better name? or more idiomatic way to do this?
    pub fn at(&self, row: usize, column: usize) -> &Color {
        &self.0[row][column]
    }

    pub fn at_mut(&mut self, row: usize, column: usize) -> &mut Color {
        &mut self.0[row][column]
    }
}

impl From<ColorMatrix> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn from(mat: ColorMatrix) -> Self {
        let mut img_buffer = ImageBuffer::new(mat.width() as u32, mat.height() as u32);

        for (row_index, row) in mat.0.into_iter().enumerate() {
            for (col_index, color) in row.into_iter().enumerate() {
                *img_buffer.get_pixel_mut(col_index as u32, row_index as u32) = color.into();
            }
        }

        img_buffer
    }
}

#[cfg(test)]
mod color_mat_tests {
    use super::*;

    #[test]
    fn create_and_compute_dimensions() {
        let mat = ColorMatrix::new(2, 3);
        assert_eq!(mat.width(), 2);
        assert_eq!(mat.height(), 3);
    }

    #[test]
    fn access_entry() {
        let mat = ColorMatrix::new(2, 3);
        let bottom_right_entry = mat.at(2, 1);
        assert_eq!(*bottom_right_entry, Color::from_rgb(0.0, 0.0, 0.0))
    }

    #[test]
    fn access_and_modify_entry() {
        let mut mat = ColorMatrix::new(2, 3);
        let bottom_right_entry = mat.at_mut(2, 1);
        assert_eq!(*bottom_right_entry, Color::from_rgb(0.0, 0.0, 0.0));

        *bottom_right_entry = Color::from_rgb(1.0, 1.0, 1.0);
        assert_eq!(*mat.at(2, 1), Color::from_rgb(1.0, 1.0, 1.0));
    }
}
