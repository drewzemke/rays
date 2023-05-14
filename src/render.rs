use crate::{
    color::{Color, ColorMatrix},
    math::{
        ray::{Intersection, Ray},
        vec3::Vec3,
        SphereAtOrigin,
    },
};

pub fn hello_sphere(output_width: u32, output_height: u32) -> ColorMatrix {
    let aspect_ratio = (output_width as f32) / (output_height as f32);

    // camera setup (all in world units)
    // currently facing down the (negative) z-axis
    let viewport_width = 4.0;
    let viewport_height = viewport_width / aspect_ratio;
    let focal_length = 2.0;

    let camera_origin = Vec3::new(0.0, 0.0, 2.0);

    let camera_right = Vec3::new(viewport_width, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, viewport_height, 0.0);
    let camera_forward = Vec3::new(0.0, 0.0, focal_length);

    let dir_for_pixel = |pix_x: u32, pix_y: u32| {
        // normalized screen coords (-1 to 1)
        let u = 2.0 * (pix_x as f32) / (output_width as f32) - 1.0;
        let v = 2.0 * (pix_y as f32) / (output_height as f32) - 1.0;

        &(&camera_forward + &(u * &camera_right)) + &(v * &camera_up)
    };

    let sphere = SphereAtOrigin::new(1.0);

    // sky color
    let zenith_col = Color::from_rgb(1.0, 1.0, 1.0);
    let nadir_col = Color::from_rgb(0.5, 0.7, 1.0);

    // create output
    let mut color_mat = ColorMatrix::new(output_width as usize, output_height as usize);

    // compute pixel values
    for pix_x in 0..output_width {
        for pix_y in 0..output_height {
            let origin = Vec3::clone(&camera_origin);
            let dir = dir_for_pixel(pix_x, pix_y).normalize();
            let ray = Ray::new(origin, dir.clone());

            let mat_entry = color_mat.at_mut(pix_y as usize, pix_x as usize);

            match ray.intersect_sphere(&sphere) {
                Some(Intersection { point: _, normal }) => {
                    let mapped_normal = 0.5 * &(&normal + &Vec3::new(1.0, 1.0, 1.0));
                    *mat_entry = mapped_normal.into();
                }
                None => {
                    // TODO: lerp!!
                    let t = 0.5 * (dir.y + 1.0);

                    let sky_color = &((1.0 - t) * &zenith_col) + &(t * &nadir_col);
                    *mat_entry = sky_color;
                }
            }
        }
    }

    color_mat
}
