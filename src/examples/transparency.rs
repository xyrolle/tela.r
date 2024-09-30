use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const SQUARE_HEIGHT: usize = 400;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    image.fill(Pixel::with_default_alpha(0, 0, 0));

    image.fill_rect(
        0,
        0,
        SQUARE_HEIGHT,
        SQUARE_HEIGHT,
        Pixel::with_default_alpha(255, 0, 0),
    );

    image.fill_rect(
        SQUARE_HEIGHT / 8,
        SQUARE_HEIGHT / 8,
        SQUARE_HEIGHT,
        SQUARE_HEIGHT,
        Pixel::new(0, 255, 0, 125),
    );

    image.fill_rect(
        SQUARE_HEIGHT / 4,
        SQUARE_HEIGHT / 4,
        SQUARE_HEIGHT,
        SQUARE_HEIGHT,
        Pixel::new(0, 0, 255, 125),
    );

    image
        .save_to_png("transparency.png")
        .expect("Failed to save PPM file");
}
