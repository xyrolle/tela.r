use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const RADIUS: usize = 50;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    image.fill(Pixel { r: 0, g: 0, b: 0 });

    image.fill_circle(WIDTH / 2, HEIGHT / 2, RADIUS, Pixel { r: 255, g: 0, b: 0 });

    image
        .save_to_ppm("circle.ppm")
        .expect("Failed to save PPM file");
}
