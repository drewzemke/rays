use crate::{
    camera::Camera,
    math::color::{Color, ColorMatrix},
    scene::Scene,
};

pub fn render(
    scene: Scene,
    camera: Camera,
    output_width: u32,
    output_height: u32,
    samples_per_pixel: u32,
    bounce_depth: u32,
) -> ColorMatrix {
    // create output
    let mut color_mat = ColorMatrix::new(output_width as usize, output_height as usize);

    // compute pixel values
    for pixel_x in 0..output_width {
        for pixel_y in 0..output_height {
            let mut accumulated_color = Color::from_rgb_f32(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let ray = camera.ray_for_pixel(pixel_x, pixel_y);
                accumulated_color = &accumulated_color + &scene.color_for_ray(ray, bounce_depth);
            }

            // gamma correction -- move to a post processing module at some point
            let avg_color = (1.0 / samples_per_pixel as f32) * &accumulated_color;
            // probably also implement a color exponential function
            let exponent = 1.0 / 2.2;
            let corrected_color = Color::from_rgb_f32(
                avg_color.r().powf(exponent),
                avg_color.g().powf(exponent),
                avg_color.b().powf(exponent),
            );

            let mat_entry = color_mat.at_mut(pixel_y as usize, pixel_x as usize);
            *mat_entry = corrected_color;
        }
    }

    color_mat
}
