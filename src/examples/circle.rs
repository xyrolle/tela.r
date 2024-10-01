use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const RADIUS: usize = 200;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    image.fill(Pixel::with_default_alpha(0, 0, 0));

    image.fill_circle(
        WIDTH / 2 - RADIUS / 2,
        HEIGHT / 2,
        RADIUS,
        Pixel::new(255, 0, 0, 255),
    );
    image.fill_circle(
        WIDTH / 2 + RADIUS / 2,
        HEIGHT / 2,
        RADIUS,
        Pixel::new(0, 0, 255, 125),
    );

    image
        .save_to_png("circle.png")
        .expect("Failed to save PPM file");
}
