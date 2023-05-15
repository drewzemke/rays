use image::{ImageBuffer, Rgb};
use rays::{
    camera::Camera,
    math::vec3::Vec3,
    render::render,
    scene::{object::sphere::Sphere, Scene},
};

fn main() {
    // window setup
    let output_width = 800;
    let output_height = 500;

    // scene setup
    let scene = Scene::new(vec![
        Box::new(Sphere::new(1.0, Vec3::new(1.0, 1.0, 0.0))),
        Box::new(Sphere::new(0.5, Vec3::new(-1.0, 0.5, -2.0))),
        Box::new(Sphere::new(0.5, Vec3::new(-2.0, 0.5, 1.0))),
        Box::new(Sphere::new(5.0, Vec3::new(5.0, 5.0, -5.0))),
    ]);

    // camera setup
    // QUESTION: should this be part of scene?
    let camera = Camera::new(
        output_width,
        output_height,
        4.0,
        4.0,
        Vec3::new(0.0, 1.0, 4.0),
    );

    // render
    let color_mat = render(scene, camera, output_width, output_height);

    // write to output
    let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = color_mat.into();
    img_buffer.save("target/debug/img_out/render.png").unwrap();
}
