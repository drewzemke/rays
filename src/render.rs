use image::ImageBuffer;

use crate::{
    color::Color,
    math::{
        ray::{Intersection, Ray},
        vec3::Vec3,
        SphereAtOrigin,
    },
};

pub fn img_hello_world() {
    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        let xf = x as f32;
        let yf = y as f32;
        image::Rgb([
            100,
            (255f32 * xf / 512f32) as u8,
            (255f32 * yf / 512f32) as u8,
        ])
    });

    img.save("target/debug/img_out/hello.png").unwrap();
}

pub fn hello_sphere() {
    // window setup
    let output_width = 800;
    let output_height = 450;
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

    // create image buffer
    let mut img_buffer = image::ImageBuffer::new(output_width, output_height);

    // compute pixel values
    for pix_x in 0..output_width {
        for pix_y in 0..output_height {
            let origin = Vec3::clone(&camera_origin);
            let dir = dir_for_pixel(pix_x, pix_y);
            let ray = Ray::new(origin, dir);

            let pixel = img_buffer.get_pixel_mut(pix_x, pix_y);

            match ray.intersect_sphere(&sphere) {
                Some(Intersection { point: _, normal }) => {
                    let mapped_normal = 0.5 * &(&normal + &Vec3::new(1.0, 1.0, 1.0));
                    let color: Color = mapped_normal.into();
                    *pixel = color.into();
                }
                None => {
                    *pixel = image::Rgb::<u8>([0, 0, 0]);
                }
            }
        }
    }

    // write to output
    img_buffer.save("target/debug/img_out/render.png").unwrap();
}
