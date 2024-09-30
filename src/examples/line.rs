use tela;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

pub fn run() {
    let mut image = tela::Image::new(WIDTH, HEIGHT);

    image.fill(tela::Pixel::with_default_alpha(0, 0, 0));

    image.draw_line(
        0,
        0,
        WIDTH / 4,
        HEIGHT,
        1,
        tela::Pixel {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
    );

    image
        .save_to_png("line.png")
        .expect("Failed to save PPM file");
}
