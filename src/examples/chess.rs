use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const RECT_HEIGHT: usize = 100;
const RECT_WIDTH: usize = 100;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    let rows = HEIGHT / RECT_HEIGHT;
    let cols = WIDTH / RECT_WIDTH;

    image.fill(Pixel::with_default_alpha(0x26, 0x19, 0x0F));

    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                image.fill_rect(
                    x * RECT_WIDTH,
                    y * RECT_HEIGHT,
                    RECT_WIDTH,
                    RECT_HEIGHT,
                    Pixel::with_default_alpha(0xE8, 0xD3, 0xB2),
                );
            }
        }
    }

    image
        .save_to_png("chess.png")
        .expect("Failed to save PPM file");
}
