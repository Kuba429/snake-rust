use crate::snake;
use piston_window::*;
use rand::Rng;
pub struct Food {
    pub x: f64,
    pub y: f64,
    cell_size: f64,
    color: [f32; 4],
}
impl Food {
    pub fn new(cell_size: f64) -> Food {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(1..30) as f64;
        let y: f64 = rng.gen_range(1..30) as f64;
        Food {
            x,
            y,
            cell_size,
            color: color::hex("B33030"),
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        rectangle(
            self.color,
            [
                self.x * self.cell_size,
                self.y * self.cell_size,
                self.cell_size,
                self.cell_size,
            ],
            con.transform,
            g,
        )
    }
    pub fn get_eaten(&mut self, snake: &mut snake::Snake) {
        &snake.add_to_tail();
        self.change_position();
    }
    pub fn change_position(&mut self) {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(1..30) as f64;
        let y: f64 = rng.gen_range(1..30) as f64;
        self.x = x.trunc();
        self.y = y.trunc();
    }
}
