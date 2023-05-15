use crate::{
    camera::Camera,
    math::{
        color::{Color, ColorMatrix},
        ray::{IntersectRay, Intersection},
        shaping::lerp,
        vec3::Vec3,
    },
    scene::object::sphere::Sphere,
};

pub fn hello_sphere(output_width: u32, output_height: u32) -> ColorMatrix {
    let camera = Camera::new(
        output_width,
        output_height,
        4.0,
        2.0,
        Vec3::new(0.0, 0.0, 2.0),
    );

    let sphere = Sphere::new(1.0);

    // sky color
    let zenith_col = Color::from_rgb(1.0, 1.0, 1.0);
    let nadir_col = Color::from_rgb(0.5, 0.7, 1.0);

    // create output
    let mut color_mat = ColorMatrix::new(output_width as usize, output_height as usize);

    // compute pixel values
    for pixel_x in 0..output_width {
        for pixel_y in 0..output_height {
            let ray = camera.ray_for_pixel(pixel_x, pixel_y);

            let mat_entry = color_mat.at_mut(pixel_y as usize, pixel_x as usize);

            match sphere.intersect_ray(&ray) {
                Some(Intersection { point: _, normal }) => {
                    let mapped_normal = 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
                    *mat_entry = mapped_normal.into();
                }
                None => {
                    // TODO: remap!!
                    let t = 0.5 * (ray.dir.y + 1.0);

                    let sky_color = lerp(t, zenith_col, nadir_col);
                    *mat_entry = sky_color;
                }
            }
        }
    }

    color_mat
}
