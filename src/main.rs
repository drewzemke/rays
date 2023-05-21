use image::{ImageBuffer, Rgb};
use rays::{
    camera::Camera,
    math::{color::Color, vec3::Vec3},
    render::render,
    scene::{
        object::{
            geometry::{plane::Plane, sphere::Sphere},
            material::lambertian::Lambertian,
            Object,
        },
        Scene,
    },
};

fn main() {
    // window setup
    let output_width = 800;
    let output_height = 500;

    // scene setup
    let sphere0 = Sphere::new(1.0, Vec3::new(1.0, 1.0, 0.0));
    let sphere1 = Sphere::new(0.5, Vec3::new(-1.0, 0.5, -2.0));
    let sphere2 = Sphere::new(0.5, Vec3::new(-2.0, 0.5, 1.0));
    let sphere3 = Sphere::new(5.0, Vec3::new(5.0, 5.0, -5.0));
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    let material = Lambertian::new(Color::from_rgb(0.5, 0.5, 0.5));

    let object0 = Object {
        geometry: &sphere0,
        material: &material,
    };
    let object1 = Object {
        geometry: &sphere1,
        material: &material,
    };
    let object2 = Object {
        geometry: &sphere2,
        material: &material,
    };
    let object3 = Object {
        geometry: &sphere3,
        material: &material,
    };
    let object4 = Object {
        geometry: &plane,
        material: &material,
    };

    let scene = Scene::new(vec![&object0, &object1, &object2, &object3, &object4]);

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
    let samples_per_pixel = 5;
    let bounce_depth = 4;

    let color_mat = render(
        scene,
        camera,
        output_width,
        output_height,
        samples_per_pixel,
        bounce_depth,
    );

    // write to output
    let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = color_mat.into();
    img_buffer.save("target/debug/img_out/render.png").unwrap();
}
