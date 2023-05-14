use image::{ImageBuffer, Rgb};
use rays::render::hello_sphere;

fn main() {
    // window setup
    let output_width = 800;
    let output_height = 500;
    let color_mat = hello_sphere(output_width, output_height);

    // write to output

    let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = color_mat.into();
    img_buffer.save("target/debug/img_out/render.png").unwrap();
}
