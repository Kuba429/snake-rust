use piston_window::*;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        let dir = match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        };
        dir
    }
}
#[derive(Clone, Copy)]
pub struct Block {
    pub x: f64,
    pub y: f64,
}
pub struct Snake {
    pub x: f64,
    pub y: f64,
    pub direction: Direction,
    cell_size: f64,
    color: [f32; 4],
    pub tail: Vec<Block>,
}
impl Snake {
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        //head
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
        );
        //tail
        for block in self.tail.iter() {
            rectangle(
                self.color,
                [
                    block.x * self.cell_size,
                    block.y * self.cell_size,
                    self.cell_size,
                    self.cell_size,
                ],
                con.transform,
                g,
            )
        }
    }
    pub fn update(&mut self) {
        //tail
        let last_block_index = self.tail.len() - 1;
        self.tail[last_block_index].x = self.x;
        self.tail[last_block_index].y = self.y;

        for i in 0..self.tail.len() - 1 {
            self.tail[i].x = self.tail[i + 1].x;
            self.tail[i].y = self.tail[i + 1].y;
        }
        //head
        match self.direction {
            Direction::Up => self.y -= 1.0,
            Direction::Down => self.y += 1.0,
            Direction::Right => self.x += 1.0,
            Direction::Left => self.x -= 1.0,
        }
        if self.x < 0.0 {
            self.x = 29.0
        } else if self.x >= 30.0 {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = 29.0
        } else if self.y >= 30.0 {
            self.y = 0.0;
        }
    }
    pub fn add_to_tail(&mut self) {
        let last_index = self.tail.len() - 1;
        let last_block = self.tail[last_index];
        self.tail.insert(0, last_block);
    }
    pub fn new(x: f64, y: f64, cell_size: f64) -> Snake {
        Snake {
            x,
            y,
            cell_size,
            direction: Direction::Right,
            color: color::hex("19282F"),
            tail: Vec::from([Block { x, y }, Block { x, y }, Block { x, y }]),
        }
    }
}
