use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Pixel { r: 0, g: 0, b: 0 }; width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn save_to_ppm(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for pixel in &self.pixels {
            writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
        }

        Ok(())
    }

    pub fn fill(&mut self, color: Pixel) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }

    pub fn fill_rect(&mut self, x0: usize, y0: usize, w: usize, h: usize, color: Pixel) {
        for dy in 0..h {
            let y = y0 + dy;
            if y < self.height {
                for dx in 0..w {
                    let x = x0 + dx;
                    if x < self.width {
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }
        }
    }

    /// Draws a filled circle.
    pub fn fill_circle(&mut self, cx: usize, cy: usize, r: usize, color: Pixel) {
        let r_sq = (r * r) as isize;
        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as isize - cx as isize;
                let dy = y as isize - cy as isize;
                if dx * dx + dy * dy <= r_sq {
                    self.pixels[y * self.width + x] = color;
                }
            }
        }
    }

    pub fn draw_line(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        thickness: usize,
        color: Pixel,
    ) {
        if x0 == x1 {
            for y in y0.min(y1)..=y0.max(y1) {
                for t in 0..thickness {
                    let offset = t as isize - (thickness / 2) as isize;
                    let x = x0 as isize + offset;
                    if x >= 0 && x < self.width as isize && y < self.height {
                        self.pixels[y * self.width + x as usize] = color;
                    }
                }
            }
            return;
        }

        let m = (y1 as f64 - y0 as f64) / (x1 as f64 - x0 as f64);
        let b = y0 as f64 - m * x0 as f64;

        for x in x0.min(x1)..=x0.max(x1) {
            let y = (m * x as f64 + b).round() as usize;

            if y < self.height {
                self.pixels[y * self.width + x] = color;

                for t in 0..thickness {
                    let offset = t as isize - (thickness / 2) as isize;
                    let ty = y as isize + offset;
                    if ty >= 0 && ty < self.height as isize {
                        self.pixels[ty as usize * self.width + x] = color;
                    }
                }
            }
        }
    }
}
