use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const RADIUS: usize = 50;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    image.fill(Pixel::with_default_alpha(0, 0, 0));

    image.fill_circle(
        WIDTH / 2,
        HEIGHT / 2,
        RADIUS,
        Pixel::with_default_alpha(255, 0, 0),
    );

    image
        .save_to_png("circle.png")
        .expect("Failed to save PPM file");
}
