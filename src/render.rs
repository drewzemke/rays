use crate::{camera::Camera, math::color::ColorMatrix, scene::Scene};

pub fn render(scene: Scene, camera: Camera, output_width: u32, output_height: u32) -> ColorMatrix {
    // create output
    let mut color_mat = ColorMatrix::new(output_width as usize, output_height as usize);

    // compute pixel values
    for pixel_x in 0..output_width {
        for pixel_y in 0..output_height {
            let ray = camera.ray_for_pixel(pixel_x, pixel_y);

            let mat_entry = color_mat.at_mut(pixel_y as usize, pixel_x as usize);
            *mat_entry = scene.color_for_ray(ray)
        }
    }

    color_mat
}
