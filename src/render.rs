use crate::{
    camera::Camera,
    math::{color::ColorMatrix, vec3::Vec3},
    scene::{object::sphere::Sphere, Scene},
};

pub fn hello_sphere(output_width: u32, output_height: u32) -> ColorMatrix {
    let camera = Camera::new(
        output_width,
        output_height,
        4.0,
        4.0,
        Vec3::new(0.0, 1.0, 4.0),
    );

    // create output
    let mut color_mat = ColorMatrix::new(output_width as usize, output_height as usize);

    let scene = Scene::new(vec![
        Sphere::new(1.0, Vec3::new(1.0, 1.0, 0.0)),
        Sphere::new(0.5, Vec3::new(-1.0, 0.5, -2.0)),
        Sphere::new(0.5, Vec3::new(-2.0, 0.5, 1.0)),
        Sphere::new(5.0, Vec3::new(5.0, 5.0, -5.0)),
    ]);

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
