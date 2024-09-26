use examples::chess;
use examples::circle;
use examples::line;
use examples::triangle;

pub mod examples {
    pub mod chess;
    pub mod circle;
    pub mod line;
    pub mod triangle;
}

fn main() {
    chess::run();
    circle::run();
    line::run();
    triangle::run();
}
