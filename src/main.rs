use std::fs;

use image::{ImageBuffer, Rgb};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rays::{
    camera::Camera,
    math::{color::Color, vec3::Vec3},
    render::render,
    scene::{
        object::{
            geometry::{plane::Plane, sphere::Sphere},
            material::{
                lambertian::Lambertian, metal::Metal, translucent::Translucent, ScatterRay,
            },
            Object,
        },
        Scene, SceneBuilder,
    },
};

fn main() {
    // window setup
    let output_width = 800;
    let output_height = 500;

    // scene setup
    let mut scene_builder = make_initial_test_scene();

    // camera setup
    // QUESTION: should this be part of scene?
    let camera = Camera::new(
        Vec3::new(13.0, 1.5, 3.0),
        Vec3::new(0.0, 0.5, 0.0),
        30.0,
        10.0,
        0.07,
        output_width,
        output_height,
    );
    scene_builder.add_camera(camera);
    let scene = scene_builder.build();

    // write_to_json(&scene, "test.json")

    write_to_json(&scene, "first_test_scene.json");

    // render
    let samples_per_pixel = 20;
    let bounce_depth = 10;

    let color_mat = render(
        scene,
        output_width,
        output_height,
        samples_per_pixel,
        bounce_depth,
    );

    // write to output
    let img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = color_mat.into();
    img_buffer.save("target/debug/img_out/render.png").unwrap();
}

const SCENE_PATH: &str = "./data";

fn write_to_json(scene: &Scene, name: &str) {
    fs::write(
        format!("{}/{}", SCENE_PATH, name),
        serde_json::to_vec(scene).unwrap(),
    )
    .unwrap();
}

// temporary until I'm happy with serde usage
fn make_initial_test_scene() -> SceneBuilder {
    let mut scene = Scene::builder();

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
        geometry: Box::new(sphere0),
        material: Box::new(lambert_pink),
    };
    let object1 = Object {
        geometry: Box::new(sphere1),
        material: Box::new(lambert_green),
    };
    let object2 = Object {
        geometry: Box::new(sphere2),
        material: Box::new(metal_yellow),
    };
    let object3 = Object {
        geometry: Box::new(sphere3),
        material: Box::new(metal_orange),
    };
    let object4 = Object {
        geometry: Box::new(plane),
        material: Box::new(lambert_gray),
    };
    let object5 = Object {
        geometry: Box::new(sphere4),
        material: Box::new(glass.clone()),
    };
    let object6 = Object {
        geometry: Box::new(sphere5),
        material: Box::new(glass),
    };

    scene.add_object(object0);
    scene.add_object(object1);
    scene.add_object(object2);
    scene.add_object(object3);
    scene.add_object(object4);
    scene.add_object(object5);
    scene.add_object(object6);

    scene
}

// temporary until I'm happy with serde usage
fn _make_tutorial_end_scene() -> SceneBuilder {
    let mut scene = Scene::builder();

    let sphere0 = Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0));
    let sphere0_mat = Translucent::new(1.5);
    scene.add_object(Object {
        geometry: Box::new(sphere0),
        material: Box::new(sphere0_mat),
    });

    let sphere1 = Sphere::new(1.0, Vec3::new(-4.0, 1.0, 0.0));
    let sphere1_mat = Lambertian::new(Color::from_rgb_f32(0.1, 0.2, 0.4));
    scene.add_object(Object {
        geometry: Box::new(sphere1),
        material: Box::new(sphere1_mat),
    });

    let sphere2 = Sphere::new(1.0, Vec3::new(4.0, 1.0, 0.0));
    let sphere2_mat = Metal::new(Color::from_rgb_f32(0.5, 0.6, 0.7), 0.0);
    scene.add_object(Object {
        geometry: Box::new(sphere2),
        material: Box::new(sphere2_mat),
    });

    let floor = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let floor_mat = Lambertian::new(Color::from_rgb_f32(0.5, 0.5, 0.5));
    scene.add_object(Object {
        geometry: Box::new(floor),
        material: Box::new(floor_mat),
    });

    let mut rng: StdRng = SeedableRng::seed_from_u64(7);
    let radius = 0.15;
    for x in -15..11 {
        for z in -11..11 {
            // generate small spheres with randomly displaced locations
            let center_offset_x = 0.8 * rng.gen::<f32>();
            let center_offset_z = 0.8 * rng.gen::<f32>();
            let center = Vec3::new(
                x as f32 + center_offset_x,
                radius,
                z as f32 + center_offset_z,
            );
            let sphere = Box::new(Sphere::new(radius, center));

            // generate material at random
            let rand_color = Color::from_rgb_f32(
                rng.gen_range(0.2..0.7),
                rng.gen_range(0.3..0.8),
                rng.gen_range(0.5..1.0),
            );
            let material: Box<dyn ScatterRay> = match rng.gen::<f32>() {
                x if x < 0.8 => Box::new(Lambertian::new(rand_color)),
                x if x < 0.95 => Box::new(Metal::new(rand_color, 0.5 * rng.gen::<f32>())),
                _ => Box::new(Translucent::new(1.5)),
            };

            scene.add_object(Object {
                geometry: sphere,
                material,
            });
        }
    }

    scene
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
