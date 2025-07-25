extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord(width), to_coord(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game::new(width, height);

    // Load font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font_path = &assets.join("fira-sans.semibold.ttf");
    let mut glyphs = window.load_font(font_path).unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key); 
        }
        event.update(|arg| {
            game.update(arg.dt);
        });
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });
    }
}
