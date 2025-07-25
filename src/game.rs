

use piston_window::types::Color;
use piston_window::*;
use rand::{rng, Rng};
use piston_window::{Glyphs, Transformed};

use crate::draw::{draw_rectangle, draw_circle, draw_grid};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.8, 0.8, 0.8, 0.5];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 10.0;
const RESTART_TIME: f64 = 3.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    pub score: u32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 1.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            score: 0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
            self.update_snake(Some(dir));
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        // Draw animated background (simple color for now)
        let grid_color: Color = [0.7, 0.7, 0.7, 0.2];
        draw_grid(grid_color, self.width, self.height, con, g);
        self.snake.draw(con, g);

        if self.food_exists {
            let food_color: Color = [1.0, 0.2, 0.2, 1.0];
            draw_circle(food_color, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        // Draw score above the game area, outside the border
        let score_text = format!("Score: {}", self.score);
        // Place score above the top border (y = -10)
        let transform = con.transform.trans(40.0, -10.0);
        text([1.0, 1.0, 0.2, 1.0], 28, &score_text, glyphs, transform, g).ok();
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
            let game_over_text = "Game Over!";
            let restart_text = "Press R to Restart";
            let instructions_text = "Arrows: Move | P: Pause | Esc: Quit";
            let t1 = con.transform.trans(60.0, 200.0);
            let t2 = con.transform.trans(60.0, 240.0);
            let t3 = con.transform.trans(60.0, 280.0);
            text([1.0, 1.0, 1.0, 1.0], 32, game_over_text, glyphs, t1, g).ok();
            text([1.0, 1.0, 1.0, 1.0], 20, restart_text, glyphs, t2, g).ok();
            text([1.0, 1.0, 1.0, 1.0], 16, instructions_text, glyphs, t3, g).ok();
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            self.score += 1;
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    
    fn add_food(&mut self) {
        let mut rng = rng();
       let  new_x = rng.random_range(1..self.width +1);
       let new_y = rng.random_range(1..self.height +1);

        while self.snake.overlap_tail(new_x, new_y) {
           
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }


    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 3.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 3.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.score = 0;
    }
}
