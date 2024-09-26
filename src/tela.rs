#![allow(non_snake_case)]

use std::fs::File;
use std::io::{self, Write};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[wasm_bindgen]
impl Pixel {
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    #[wasm_bindgen]
    pub fn with_default_alpha(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }
}

#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Pixel::with_default_alpha(0, 0, 0); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen]
    pub fn get_pixels(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.pixels.len() * 4);
        for pixel in &self.pixels {
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
            data.push(pixel.a);
        }
        data
    }

    #[wasm_bindgen]
    pub fn get_pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr() as *const u8
    }

    #[wasm_bindgen]
    pub fn get_pixels_length(&self) -> usize {
        self.pixels.len() * 4 // Each pixel has 3 u8 components (r, g, b, a)
    }

    #[wasm_bindgen]
    pub fn fill(&mut self, color: Pixel) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }

    #[wasm_bindgen]
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

    #[wasm_bindgen]
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

    #[wasm_bindgen]
    pub fn render(&mut self, dt: f32) {
        self.fill(Pixel::with_default_alpha(0, 0, 0));

        let size = 200.0;

        let angle = (dt * 90.0) % 360.0;

        let alpha = (127.5 * (1.0 + (dt * 2.0 * std::f32::consts::PI).sin())) as u8;

        let vertices = [
            (0.0, -size / 2.0),
            (size * 0.866, size / 2.0),
            (-size * 0.866, size / 2.0),
        ];

        let triangle_center_x = (vertices[0].0 + vertices[1].0 + vertices[2].0) / 3.0;
        let triangle_center_y = (vertices[0].1 + vertices[1].1 + vertices[2].1) / 3.0;

        let centered_vertices: Vec<(f32, f32)> = vertices
            .iter()
            .map(|&(x, y)| (x - triangle_center_x, y - triangle_center_y))
            .collect();

        let rotated_vertices: Vec<(f32, f32)> = centered_vertices
            .iter()
            .map(|&(x, y)| rotate_point(x, y, angle))
            .collect();

        let canvas_center_x = self.width as f32 / 2.0;
        let canvas_center_y = self.height as f32 / 2.0;

        let translated_vertices: Vec<(f32, f32)> = rotated_vertices
            .iter()
            .map(|&(x, y)| (x + canvas_center_x, canvas_center_y - y))
            .collect();

        self.draw_filled_triangle(
            translated_vertices[0].0,
            translated_vertices[0].1,
            translated_vertices[1].0,
            translated_vertices[1].1,
            translated_vertices[2].0,
            translated_vertices[2].1,
            Pixel::new(255, 0, 0, alpha),
        );
    }

    #[wasm_bindgen]
    pub fn draw_line(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        thickness: usize,
        color: Pixel,
    ) {
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut e2;

        loop {
            for t in 0..thickness {
                let offset = t as isize - (thickness / 2) as isize;
                let x = x0 + offset * sy;
                let y = y0 - offset * sx;
                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    self.pixels[y as usize * self.width + x as usize] = color;
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }
            e2 = err;
            if e2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if e2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }

    #[wasm_bindgen]
    pub fn draw_filled_triangle(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Pixel,
    ) {
        let mut vertices = [(x0, y0), (x1, y1), (x2, y2)];

        vertices.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let (x0, y0) = vertices[0];
        let (x1, y1) = vertices[1];
        let (x2, y2) = vertices[2];

        let inv_slope1 = if y1 != y0 { (x1 - x0) / (y1 - y0) } else { 0.0 };
        let inv_slope2 = if y2 != y0 { (x2 - x0) / (y2 - y0) } else { 0.0 };

        let mut x_start = x0;
        let mut x_end = x0;

        let mut y = y0.round() as i32;
        let y_end = y1.round() as i32;

        while y < y_end {
            let x_s = x_start.round() as i32;
            let x_e = x_end.round() as i32;

            let (x_s, x_e) = if x_s > x_e { (x_e, x_s) } else { (x_s, x_e) };

            for x in x_s..=x_e {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    self.pixels[y as usize * self.width + x as usize] = color;
                }
            }

            x_start += inv_slope1;
            x_end += inv_slope2;
            y += 1;
        }

        let inv_slope1 = if y2 != y1 { (x2 - x1) / (y2 - y1) } else { 0.0 };

        x_start = x1;
        y = y1.round() as i32;
        let y_end = y2.round() as i32;

        while y < y_end {
            let x_s = x_start.round() as i32;
            let x_e = x_end.round() as i32;

            let (x_s, x_e) = if x_s > x_e { (x_e, x_s) } else { (x_s, x_e) };

            for x in x_s..=x_e {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    self.pixels[y as usize * self.width + x as usize] = color;
                }
            }

            x_start += inv_slope1;
            x_end += inv_slope2;
            y += 1;
        }
    }
}

impl Image {
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
}

fn rotate_point(x: f32, y: f32, angle_deg: f32) -> (f32, f32) {
    let angle_rad = angle_deg.to_radians();
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let x_rot = x * cos_theta - y * sin_theta;
    let y_rot = x * sin_theta + y * cos_theta;

    (x_rot, y_rot)
}
