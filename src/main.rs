use examples::chess;
use examples::circle;
use examples::line;

mod examples {
    pub mod chess;
    pub mod circle;
    pub mod line;
}

fn main() {
    chess::run();
    circle::run();
    line::run();
}
