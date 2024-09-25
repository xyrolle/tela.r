use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const RECT_HEIGHT: usize = 100;
const RECT_WIDTH: usize = 100;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    let rows = HEIGHT / RECT_HEIGHT;
    let cols = WIDTH / RECT_WIDTH;

    image.fill(Pixel {
        r: 0x26,
        g: 0x19,
        b: 0x0F,
    });

    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                image.fill_rect(
                    x * RECT_WIDTH,
                    y * RECT_HEIGHT,
                    RECT_WIDTH,
                    RECT_HEIGHT,
                    Pixel {
                        r: 0xE8,
                        g: 0xD3,
                        b: 0xB2,
                    },
                );
            }
        }
    }

    image
        .save_to_ppm("chess.ppm")
        .expect("Failed to save PPM file");
}
