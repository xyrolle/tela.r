use tela::{Image, Pixel};

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

pub fn run() {
    let mut image = Image::new(WIDTH, HEIGHT);

    image.fill(Pixel { r: 0, g: 0, b: 0 });

    image.draw_line(
        0,
        0,
        WIDTH - 1,
        HEIGHT - 1,
        16,
        Pixel { r: 255, g: 0, b: 0 },
    );

    image
        .save_to_ppm("line.ppm")
        .expect("Failed to save PPM file");
}
