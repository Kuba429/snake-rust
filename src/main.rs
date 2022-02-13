extern crate piston_window;
mod food;
mod game;
mod snake;
use piston_window::*;

fn main() {
    let mut game = game::Game::new(800, 800);
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [game.width, game.height])
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            game.draw_all(&c, g);
        });
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.handle_key_press(key)
        }

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
