use image::{ImageBuffer, Rgb};
use rays::{
    camera::Camera,
    math::{color::Color, vec3::Vec3},
    render::render,
    scene::{
        object::{
            geometry::{plane::Plane, sphere::Sphere},
            material::{lambertian::Lambertian, metal::Metal, translucent::Translucent},
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
    let sphere4 = Sphere::new(0.75, Vec3::new(-0.75, 0.75, 2.0));
    let mut sphere5 = Sphere::new(0.65, Vec3::new(-0.75, 0.75, 2.0));
    sphere5.flip_orientation();

    // let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    let lambert_gray = Lambertian::new(Color::from_rgb_f32(0.5, 0.5, 0.5));
    let lambert_pink = Lambertian::new(Color::from_rgb_u8(255, 121, 198));
    // let lambert_purple = Lambertian::new(Color::from_rgb_u8(189, 147, 249));
    let lambert_green = Lambertian::new(Color::from_rgb_u8(80, 250, 123));
    let metal_yellow = Metal::new(Color::from_rgb_u8(241, 250, 140), 0.4);
    let metal_orange = Metal::new(Color::from_rgb_u8(255, 184, 108), 0.3);

    let glass = Translucent::new(1.5);

    let object0 = Object {
        geometry: &sphere0,
        material: &lambert_pink,
    };
    let object1 = Object {
        geometry: &sphere1,
        material: &lambert_green,
    };
    let object2 = Object {
        geometry: &sphere2,
        material: &metal_yellow,
    };
    let object3 = Object {
        geometry: &sphere3,
        material: &metal_orange,
    };
    let object4 = Object {
        geometry: &plane,
        material: &lambert_gray,
    };
    let object5 = Object {
        geometry: &sphere4,
        material: &glass,
    };
    let object6 = Object {
        geometry: &sphere5,
        material: &glass,
    };

    let scene = Scene::new(vec![
        &object0, &object1, &object2, &object3, &object4, &object5, &object6,
    ]);

    // camera setup
    // QUESTION: should this be part of scene?
    let camera = Camera::new(
        Vec3::new(0.0, 0.7, 6.0),
        Vec3::new(0.0, 1.2, 0.0),
        60.0,
        6.0,
        0.1,
        output_width,
        output_height,
    );

    // render
    let samples_per_pixel = 20;
    let bounce_depth = 10;

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

// TODO: find a better way to manage this!
// steel balls scene
// let sphere0 = Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0));
// let sphere1 = Sphere::new(1.0, Vec3::new(2.0, 1.0, 0.0));
// let sphere2 = Sphere::new(1.0, Vec3::new(1.0, 1.0, f32::sqrt(3.0)));
// let sphere3 = Sphere::new(
//     1.0,
//     Vec3::new(1.0, 2.0 * f32::sqrt(6.0) / 3.0 + 1.0, f32::sqrt(3.0) / 3.0),
// );
// let plane = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

// let ball_mat = Metal::new(Color::from_rgb_u8(241, 231, 221), 0.7);
// let floor_mat = Metal::new(Color::from_rgb_u8(8, 11, 14), 0.5);

// let object0 = Object {
//     geometry: &sphere0,
//     material: &ball_mat,
// };
// let object1 = Object {
//     geometry: &sphere1,
//     material: &ball_mat,
// };
// let object2 = Object {
//     geometry: &sphere2,
//     material: &ball_mat,
// };
// let object3 = Object {
//     geometry: &sphere3,
//     material: &ball_mat,
// };
// let object4 = Object {
//     geometry: &plane,
//     material: &floor_mat,
// };

// let scene = Scene::new(vec![&object0, &object1, &object2, &object3, &object4]);
