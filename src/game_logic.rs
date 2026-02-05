use rand::{rng, Rng};
use crate::snake::{Direction, Snake};
use crate::storage::{GameData, Difficulty};

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    paused: bool,
    waiting_time: f64,
    score: u32,
    high_score: u32,
    difficulty: Difficulty,
    game_data: GameData,
    combo: u32,
    combo_time: f64,
    food_eaten: u32,
    play_time: f64,
    should_beep: bool,
    last_score_popup: Option<u32>,
    popup_time: f64,
}

const PROGRESSIVE_SPEED_INCREASE: f64 = 0.002;

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let game_data = GameData::load();
        let high_score = game_data.high_score;
        let difficulty = game_data.difficulty;
        
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            paused: false,
            score: 0,
            high_score,
            difficulty,
            game_data,
            combo: 0,
            combo_time: 0.0,
            food_eaten: 0,
            play_time: 0.0,
            should_beep: false,
            last_score_popup: None,
            popup_time: 0.0,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn combo(&self) -> u32 {
        self.combo
    }

    pub fn food_eaten(&self) -> u32 {
        self.food_eaten
    }

    pub fn play_time(&self) -> f64 {
        self.play_time
    }

    pub fn should_beep(&mut self) -> bool {
        let beep = self.should_beep;
        self.should_beep = false;
        beep
    }

    pub fn score_popup(&self) -> Option<(u32, f64)> {
        self.last_score_popup.map(|score| (score, self.popup_time))
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        
        if self.food_x >= width {
            self.food_x = width - 1;
        }
        if self.food_y >= height {
            self.food_y = height - 1;
        }
        
        let (head_x, head_y) = self.snake.head_position();
        if head_x >= width || head_y >= height || head_x < 0 || head_y < 0 {
            self.game_over = true;
        }
    }

    pub fn change_difficulty(&mut self) {
        self.difficulty = self.difficulty.next();
        self.game_data.difficulty = self.difficulty;
        self.game_data.save();
    }

    pub fn handle_direction(&mut self, direction: GameDirection) {
        if self.game_over || self.paused {
            return;
        }

        let dir = match direction {
            GameDirection::Up => Direction::Up,
            GameDirection::Down => Direction::Down,
            GameDirection::Left => Direction::Left,
            GameDirection::Right => Direction::Right,
        };

        if dir == self.snake.head_direction().opposite() {
            return;
        }
        
        self.snake.set_direction(dir);
    }

    pub fn get_cell(&self, x: i32, y: i32) -> CellType {
        let (head_x, head_y) = self.snake.head_position();
        if x == head_x && y == head_y {
            return CellType::SnakeHead;
        }

        if self.snake.overlap_tail(x, y) {
            return CellType::SnakeBody;
        }

        if self.food_exists && x == self.food_x && y == self.food_y {
            return CellType::Food;
        }

        CellType::Empty
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if !self.game_over && !self.paused {
            self.play_time += delta_time;
        }

        // Update popup timer
        if self.popup_time > 0.0 {
            self.popup_time -= delta_time;
            if self.popup_time <= 0.0 {
                self.last_score_popup = None;
            }
        }

        if self.game_over || self.paused {
            return;
        }

        if self.combo_time > 0.0 {
            self.combo_time -= delta_time;
            if self.combo_time <= 0.0 {
                self.combo = 0;
            }
        }

        if !self.food_exists {
            self.add_food();
        }

        let speed_modifier = 1.0 - (self.score as f64 * PROGRESSIVE_SPEED_INCREASE).min(0.5);
        let moving_period = self.difficulty.speed() * speed_modifier;

        if self.waiting_time > moving_period {
            self.update_snake();
        }
    }

    fn update_snake(&mut self) {
        if self.check_if_snake_alive() {
            self.snake.move_forward(None);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            
            if self.combo_time > 0.0 {
                self.combo += 1;
            } else {
                self.combo = 1;
            }
            self.combo_time = 2.0;
            
            let points = if self.combo > 1 {
                self.combo.min(5)
            } else {
                1
            };
            self.score += points;
            self.food_eaten += 1;
            
            // Trigger beep and score popup
            self.should_beep = true;
            self.last_score_popup = Some(points);
            self.popup_time = 1.0;
            
            if self.score > self.high_score {
                self.high_score = self.score;
                self.game_data.high_score = self.high_score;
                self.game_data.save();
            }
        }
    }

    fn check_if_snake_alive(&self) -> bool {
        let (next_x, next_y) = self.snake.next_head(None);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        // Allow movement up to the boundary (0 to width-1, 0 to height-1)
        next_x >= 0 && next_y >= 0 && next_x < self.width && next_y < self.height
    }

    fn add_food(&mut self) {
        let mut rng = rng();
        // Food can appear anywhere in the play area
        let mut new_x = rng.random_range(0..self.width);
        let mut new_y = rng.random_range(0..self.height);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.random_range(0..self.width);
            new_y = rng.random_range(0..self.height);
        }
        
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
}

pub enum CellType {
    Empty,
    SnakeHead,
    SnakeBody,
    Food,
}

pub enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}
