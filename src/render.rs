use crate::{
    math::{
        color::{Color, ColorMatrix},
        ray::Ray,
    },
    scene::Scene,
};

pub fn render(
    scene: Scene,
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
            let mut accumulated_color = Color::from_rgb_u8(0, 0, 0);
            for _ in 0..samples_per_pixel {
                let ray = scene.camera.ray_for_pixel(pixel_x, pixel_y);

                accumulated_color = &accumulated_color + &color_for_ray(&scene, &ray, bounce_depth);
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

fn color_for_ray(scene: &Scene, ray: &Ray, bounce_depth: u32) -> Color {
    if bounce_depth == 0 {
        return Color::from_rgb_u8(0, 0, 0);
    }

    let closest_intersection = scene.intersect_ray(ray);

    match closest_intersection {
        Some((ref intersection, object)) => {
            match object.material.scatter_ray(ray, intersection) {
                Some((scattered_ray, reflection_color)) => {
                    reflection_color * &color_for_ray(scene, &scattered_ray, bounce_depth - 1)
                }
                // The scattering algorithm decided to absorb the ray, so return black
                None => Color::from_rgb_u8(0, 0, 0),
            }
        }
        // No intersections, so query the sky for a color
        // TODO: scene need to expose sky so this can be called from render
        None => scene.sky.sky_color_for_direction(&ray.dir),
    }
}
