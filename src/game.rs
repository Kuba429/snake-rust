use crate::food::Food;
use crate::snake::{Direction, Snake};
use piston_window::*;
pub struct Game {
    pub width: u32,
    pub height: u32,
    pub cell_size: f64,
    pub grid_size: f64,
    middle: f64,
    time_waited: f64,
    pub fps: f64,
    pub snake: Snake,
    pub food: Food,
    pub is_over: bool,
    background_color: [f32; 4],
    grid_color: [f32; 4],
}
impl Game {
    pub fn draw_all(&mut self, con: &Context, g: &mut G2d) {
        clear(self.background_color, g);
        self.draw_grid(con, g);
        self.snake.draw(&con, g);
        self.food.draw(&con, g);
    }
    fn draw_grid(&self, con: &Context, g: &mut G2d) {
        for i in 1..=self.grid_size as u32 {
            let mut from: [f64; 2] = [0.0, self.cell_size * i as f64];
            let mut to: [f64; 2] = [self.width as f64, self.cell_size * i as f64];
            line_from_to(self.grid_color, 0.5, from, to, con.transform, g);

            from = [self.cell_size * i as f64, 0.0];
            to = [self.cell_size * i as f64, self.height as f64];
            line_from_to(self.grid_color, 0.5, from, to, con.transform, g);
        }
    }
    pub fn update(&mut self, delta: f64) {
        if self.is_over {
            return;
        };
        self.time_waited += delta;
        if self.time_waited >= 1.0 / self.fps {
            self.snake.update();
            self.time_waited = 0.0;
            self.check_collision();
        };
    }
    pub fn handle_key_press(&mut self, key: Key) {
        if self.is_over {
            if key == Key::R {
                self.is_over = false;
                self.snake = Snake::new(self.middle, self.middle, self.cell_size as f64);
            }
            return;
        };
        let new_dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if new_dir.is_some() {
            let dir: Direction = new_dir.unwrap();
            if self.snake.direction != dir.opposite() {
                self.snake.direction = dir;
            };
            self.snake.update();
            self.check_collision();
        } else {
            self.snake.add_to_tail();
        }
    }
    pub fn check_collision(&mut self) {
        for block in &self.snake.tail {
            if block.x == self.snake.x && block.y == self.snake.y {
                println!("game over");
                self.is_over = true;
            }
        }
        if self.snake.x == self.food.x && self.snake.y == self.food.y {
            self.food.get_eaten(&mut self.snake);
        }
    }
    pub fn new(width: u32, height: u32) -> Game {
        let grid_size = 30.0;
        let cell_size = width as f64 / grid_size;
        let mut middle: f64 = grid_size / 2.0;
        middle = middle.round();
        Game {
            width,
            height,
            cell_size: cell_size as f64,
            grid_size: grid_size as f64,
            middle,
            time_waited: 0.0,
            fps: 4.0,
            snake: Snake::new(middle, middle, cell_size as f64),
            food: Food::new(cell_size as f64),
            is_over: false,
            background_color: color::hex("D3ECA7"),
            grid_color: color::hex("A1B57D"),
        }
    }
}
