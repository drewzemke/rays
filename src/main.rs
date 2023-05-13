use image::ImageBuffer;

fn main() {
    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        let xf = x as f32;
        let yf = y as f32;
        image::Rgb([
            100,
            (255f32 * xf / 512f32) as u8,
            (255f32 * yf / 512f32) as u8,
        ])
    });

    img.save("hello.png").unwrap();
}
