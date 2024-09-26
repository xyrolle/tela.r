use tela;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

pub fn run() {
    let mut image = tela::Image::new(WIDTH, HEIGHT);

    image.fill(tela::Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    });

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
        .save_to_ppm("line.ppm")
        .expect("Failed to save PPM file");
}
