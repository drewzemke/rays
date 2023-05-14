use crate::math::{ray::Ray, vec3::Vec3};

// camera manages the transformation between screen space and world space
// it takes in camera location and orientation and image output dimensions,
// and generates rays that interact with the scene
#[derive(Debug, Clone)]
pub struct Camera {
    output_width: u32,
    output_height: u32,
    position: Vec3,
    camera_forward: Vec3,
    camera_right: Vec3,
    camera_up: Vec3,
}

impl Camera {
    pub fn new(
        output_width: u32,
        output_height: u32,
        viewport_width: f32,
        focal_length: f32,
        location: Vec3,
    ) -> Camera {
        // camera setup (all in world units)
        // currently facing down the (negative) z-axis
        let aspect_ratio = (output_width as f32) / (output_height as f32);
        let viewport_height = viewport_width / aspect_ratio;

        let camera_right = Vec3::new(viewport_width, 0.0, 0.0);
        let camera_up = Vec3::new(0.0, viewport_height, 0.0);
        let camera_forward = Vec3::new(0.0, 0.0, focal_length);

        Camera {
            output_width,
            output_height,
            position: location,
            camera_forward,
            camera_right,
            camera_up,
        }
    }

    fn dir_for_pixel(&self, pixel_x: u32, pixel_y: u32) -> Vec3 {
        // normalized screen coords (-1 to 1)
        let u = 2.0 * (pixel_x as f32) / (self.output_width as f32) - 1.0;
        let v = 2.0 * (pixel_y as f32) / (self.output_height as f32) - 1.0;

        self.camera_forward + u * self.camera_right + v * self.camera_up
    }

    pub fn ray_for_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let dir = self.dir_for_pixel(pixel_x, pixel_y).normalize();
        Ray {
            origin: self.position,
            dir,
        }
    }
}
