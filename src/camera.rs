use std::f32::consts::PI;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::math::{ray::Ray, vec3::Vec3};

// camera manages the transformation between screen space and world space
// it takes in camera location and orientation and image output dimensions,
// and generates rays that interact with the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    output_width: u32,
    output_height: u32,
    position: Vec3,
    camera_forward: Vec3,
    camera_right: Vec3,
    camera_up: Vec3,
    aperture_width: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        // horizontal fov
        field_of_view_degrees: f32,
        focus_distance: f32,
        aperture_width: f32,
        output_width: u32,
        output_height: u32,
    ) -> Camera {
        // calculate the camera frame
        let global_up = Vec3::new(0.0, 1.0, 0.0);
        let camera_forward_unit = (&target - &position).normalize();
        let camera_right_unit = Vec3::cross(&camera_forward_unit, &global_up).normalize();
        let camera_up_unit = Vec3::cross(&camera_right_unit, &camera_forward_unit);

        // only need to set horizontal and vertical widths, keep the camera z-length as 1
        let aspect_ratio = (output_width as f32) / (output_height as f32);
        let viewport_width = focus_distance * (field_of_view_degrees / 2.0).to_radians().tan();
        let viewport_height = viewport_width / aspect_ratio;

        let camera_right = viewport_width * &camera_right_unit;
        let camera_up = viewport_height * &camera_up_unit;
        let camera_forward = focus_distance * &camera_forward_unit;

        Camera {
            output_width,
            output_height,
            position,
            camera_forward,
            camera_right,
            camera_up,
            aperture_width,
        }
    }

    pub fn ray_for_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let mut rng = thread_rng();
        // QUESTION: is it okay to use the same two random values for two different purposes?
        let s: f32 = rng.gen();
        let t: f32 = rng.gen();

        let target = {
            // QUESTION: also, do we still need to put noise here if we're using defocus blur?
            // normalized screen coords (-1 to 1)
            let u = 2.0 * (pixel_x as f32 + s) / (self.output_width as f32) - 1.0;
            // pixel_y traverses from top to bottom, so negate
            let v = -(2.0 * (pixel_y as f32 + t) / (self.output_height as f32) - 1.0);

            // FIXME: idk this just looks gross
            &self.position
                + &(&(&self.camera_forward + &(u * &self.camera_right)) + &(v * &self.camera_up))
        };

        let origin_offset_x = self.aperture_width * s.sqrt() * (2.0 * PI * t).cos();
        let origin_offset_y = self.aperture_width * s.sqrt() * (2.0 * PI * t).sin();

        // obtain unit vectors for right and up, then linear combo with offsets, then add to origin
        let origin_offset = &(origin_offset_x * &self.camera_right.normalize())
            + &(origin_offset_y * &self.camera_up.normalize());

        let origin = &self.position + &origin_offset;
        let dir = (&target - &origin).normalize();
        Ray { origin, dir }
    }
}
