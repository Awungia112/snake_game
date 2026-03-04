use rand::{rng, Rng};
use crate::snake::{Direction, Snake};
use crate::storage::{GameData, Difficulty};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}

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
// New fields
    obstacles: Vec<Obstacle>,
    powerups: Vec<Powerup>,
    active_effects: HashMap<PowerupType, f64>,
    // Settings & Theme
    pub theme_index: usize,
    pub spawn_timer: f64,
    pub powerup_timer: f64,
    pub settings: GameSettings,
}

#[derive(Clone, Copy)]
pub struct GameSettings {
    pub obstacle_interval: f64, // Seconds between spawns
    pub powerup_interval: f64,  // Seconds between tick checks? Or separate interval?
    // Let's use a single timer for now or separate?
    // User said "obstacles should occur in 5s". Powerups "should also occur".
    // Let's use one common spawn ticker or separate. Separate gives better control.
    pub powerup_chance: f64,    // Chance to spawn powerup INSTEAD of obstacle? Or independent?
    // "Ghost rates should be controllable"
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            obstacle_interval: 5.0,
            powerup_interval: 10.0, // Powerups might be rarer? Or maybe interval is purely chance based check?
            powerup_chance: 0.2,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Powerup {
    pub x: i32,
    pub y: i32,
    pub kind: PowerupType,
    pub duration: f64, // Duration of the effect when picked up
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerupType {
    Ghost,
    Multiplier,
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
            game_data: game_data.clone(),
            combo: 0,
            combo_time: 0.0,
            food_eaten: 0,
            play_time: 0.0,
            should_beep: false,
            last_score_popup: None,
            popup_time: 0.0,
            obstacles: Vec::new(),
            powerups: Vec::new(),
            active_effects: HashMap::new(),
            theme_index: game_data.theme_index,
            spawn_timer: 0.0,
            powerup_timer: 0.0,
            settings: GameSettings {
                obstacle_interval: game_data.obstacle_interval,
                powerup_interval: game_data.powerup_interval,
                powerup_chance: 0.2,
            },
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
    
    pub fn cycle_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % crate::ui::THEMES.len();
        self.game_data.theme_index = self.theme_index;
        self.game_data.save();
    }
    
    pub fn change_direction(&mut self, direction: GameDirection) {
        let new_dir = match direction {
            GameDirection::Up => Direction::Up,
            GameDirection::Down => Direction::Down,
            GameDirection::Left => Direction::Left,
            GameDirection::Right => Direction::Right,
        };
        self.snake.set_direction(new_dir);
    }
    
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
    
    pub fn adjust_spawn_rates(&mut self) {
        // Cycle through preset spawn rates
        // Current: 5s obstacles, 10s powerups
        // Fast: 3s obstacles, 5s powerups
        // Slow: 10s obstacles, 20s powerups
        // Off: 0s (disabled)
        
        if self.settings.obstacle_interval == 5.0 {
            // Fast
            self.settings.obstacle_interval = 3.0;
            self.settings.powerup_interval = 5.0;
        } else if self.settings.obstacle_interval == 3.0 {
            // Slow
            self.settings.obstacle_interval = 10.0;
            self.settings.powerup_interval = 20.0;
        } else if self.settings.obstacle_interval == 10.0 {
            // Off
            self.settings.obstacle_interval = 0.0;
            self.settings.powerup_interval = 0.0;
        } else {
            // Back to Normal
            self.settings.obstacle_interval = 5.0;
            self.settings.powerup_interval = 10.0;
        }
    }
    
    pub fn increase_obstacle_interval(&mut self) {
        self.settings.obstacle_interval = (self.settings.obstacle_interval + 1.0).min(30.0);
        self.game_data.obstacle_interval = self.settings.obstacle_interval;
        self.game_data.save();
    }
    
    pub fn decrease_obstacle_interval(&mut self) {
        self.settings.obstacle_interval = (self.settings.obstacle_interval - 1.0).max(0.0);
        self.game_data.obstacle_interval = self.settings.obstacle_interval;
        self.game_data.save();
    }
    
    pub fn increase_powerup_interval(&mut self) {
        self.settings.powerup_interval = (self.settings.powerup_interval + 1.0).min(60.0);
        self.game_data.powerup_interval = self.settings.powerup_interval;
        self.game_data.save();
    }
    
    pub fn decrease_powerup_interval(&mut self) {
        self.settings.powerup_interval = (self.settings.powerup_interval - 1.0).max(0.0);
        self.game_data.powerup_interval = self.settings.powerup_interval;
        self.game_data.save();
    }
    
    pub fn get_spawn_preset_name(&self) -> &str {
        if self.settings.obstacle_interval == 0.0 {
            "Off"
        } else if self.settings.obstacle_interval == 3.0 {
            "Fast"
        } else if self.settings.obstacle_interval == 5.0 {
            "Normal"
        } else if self.settings.obstacle_interval == 10.0 {
            "Slow"
        } else {
            "Custom"
        }
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

    pub fn head_position(&self) -> (i32, i32) {
        self.snake.head_position()
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
        
        for obstacle in &self.obstacles {
            if obstacle.x == x && obstacle.y == y {
                return CellType::Obstacle;
            }
        }
        
        for powerup in &self.powerups {
            if powerup.x == x && powerup.y == y {
                match powerup.kind {
                    PowerupType::Ghost => return CellType::PowerupGhost,
                    PowerupType::Multiplier => return CellType::PowerupMultiplier,
                }
            }
        }

        CellType::Empty
    }

    pub fn obstacles(&self) -> &Vec<Obstacle> {
        &self.obstacles
    }

    pub fn powerups(&self) -> &Vec<Powerup> {
        &self.powerups
    }
    
    pub fn active_effects(&self) -> &HashMap<PowerupType, f64> {
        &self.active_effects
    }
    
    pub fn is_ghost_active(&self) -> bool {
        self.active_effects.contains_key(&PowerupType::Ghost)
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
        
        self.update_spawners(delta_time);
        
        // Update active effects
        let mut effects_to_remove = Vec::new();
        for (effect, timer) in self.active_effects.iter_mut() {
            *timer -= delta_time;
            if *timer <= 0.0 {
                effects_to_remove.push(*effect);
            }
        }
        for effect in effects_to_remove {
            self.active_effects.remove(&effect);
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
        self.snake.move_forward(None);
        
        let (head_x, head_y) = self.snake.head_position();
        
        // Check for Powerup collision
        let mut powerup_index = None;
        for (i, p) in self.powerups.iter().enumerate() {
            if p.x == head_x && p.y == head_y {
                powerup_index = Some(i);
                break;
            }
        }
        if let Some(index) = powerup_index {
            let p = self.powerups.remove(index);
            self.active_effects.insert(p.kind, p.duration);
            self.should_beep = true; 
        }
        
        // Check for game over conditions
        let wall_collision = head_x < 0 || head_y < 0 || head_x >= self.width || head_y >= self.height;
        let self_collision = !self.is_ghost_active() && self.snake.has_self_collision();
        
        let mut obstacle_collision = false;
        if !self.is_ghost_active() {
            for obs in &self.obstacles {
                if obs.x == head_x && obs.y == head_y {
                    obstacle_collision = true;
                    break;
                }
            }
        }

        if wall_collision || self_collision || obstacle_collision {
            self.game_over = true;
        } else {
            self.check_eating();
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
            
            let mut points = if self.combo > 1 {
                self.combo.min(5)
            } else {
                1
            };
            
            if self.active_effects.contains_key(&PowerupType::Multiplier) {
                points *= 2;
            }
            
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



    fn add_food(&mut self) {
        let _rng = rng();
        
        // 1. Spawn Food
        let (fx, fy) = self.get_random_free_pos();
        self.food_x = fx;
        self.food_y = fy;
        self.food_exists = true;
    }
    
    fn update_spawners(&mut self, delta_time: f64) {
        // Obstacle Spawner
        if self.settings.obstacle_interval > 0.0 {
            self.spawn_timer += delta_time;
            if self.spawn_timer >= self.settings.obstacle_interval {
                self.spawn_timer = 0.0;
                let (ox, oy) = self.get_random_free_pos();
                self.obstacles.push(Obstacle { x: ox, y: oy });
            }
        }
        
        // Powerup Spawner
        if self.settings.powerup_interval > 0.0 {
            self.powerup_timer += delta_time;
            if self.powerup_timer >= self.settings.powerup_interval {
                self.powerup_timer = 0.0;
                let mut rng = rng();
                if rng.random_bool(self.settings.powerup_chance) {
                    let (px, py) = self.get_random_free_pos();
                    let kind = if rng.random_bool(0.5) { PowerupType::Ghost } else { PowerupType::Multiplier };
                    self.powerups.push(Powerup { x: px, y: py, kind, duration: 10.0 });
                }
            }
        }
    }
    
    fn get_random_free_pos(&self) -> (i32, i32) {
        let mut rng = rng();
        // Safety break to prevent infinite loops if board is full
        for _ in 0..100 {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            if !self.is_pos_occupied(x, y) {
                return (x, y);
            }
        }
        // Fallback: Just return a random pos if we can't find a free one quickly
        (rng.random_range(0..self.width), rng.random_range(0..self.height))
    }
    
    fn is_pos_occupied(&self, x: i32, y: i32) -> bool {
        // occupied by snake body
        if self.snake.overlap_tail(x, y) { return true; }
        // occupied by snake head 
        let (hx, hy) = self.snake.head_position();
        if x == hx && y == hy { return true; }
        // occupied by food
        if self.food_exists && x == self.food_x && y == self.food_y { return true; }
        // occupied by obstacle
        for o in &self.obstacles { if o.x == x && o.y == y { return true; } }
        // occupied by powerup
        for p in &self.powerups { if p.x == x && p.y == y { return true; } }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_wall_collision() {
        let width = 10;
        let height = 10;
        let mut game = Game::new(width, height);
        
        // Manually position snake head at x=9 (right edge), y=5 facing Right
        // Requires modifying internal state which isn't easily accessible without helpers 
        // or just playing the game to that point. 
        // Since we don't have public setters for testing, we can simulate moves.
        
        // Snake Head starts at x=4 (created with new(2,2) -> head at 2+2=4)
        // Distance to wall (10) is 6 blocks.
        // Moves: 5 (x=5), 6 (x=6), 7 (x=7), 8 (x=8), 9 (x=9) -> 5 safe moves.
        // 6th move hits x=10.
        
        for _ in 0..5 {
            game.update_snake();
            assert!(!game.game_over, "Game should not be over yet");
        }
        
        // Head should be at x=9 now (2 + 7 = 9)
        let (head_x, _) = game.snake.head_position();
        assert_eq!(head_x, 9);
        
        // One more move should hit the wall (x=10) and trigger game over
        game.update_snake();
        
        // Verify game over
        assert!(game.game_over, "Game should be over after hitting wall");
        
        // Verify head position is at the wall (out of bounds)
        let (head_x, _) = game.snake.head_position();
        assert_eq!(head_x, 10);
    }

    #[test]
    fn test_upper_wall_collision() {
        let width = 10;
        let height = 10;
        let mut game = Game::new(width, height);
        
        // Initial head at (2, 2)
        // Needs to move UP to collide with y < 0
        // y: 2 -> 1 -> 0 -> -1
        
        game.handle_direction(crate::game_logic::GameDirection::Up);
        
        // Move 1: y=1
        game.update_snake();
        assert!(!game.game_over, "Should be valid at y=1");
        
        // Move 2: y=0 (Top edge inside board)
        game.update_snake();
        assert!(!game.game_over, "Should be valid at y=0");
        let (_, head_y) = game.snake.head_position();
        assert_eq!(head_y, 0, "Head should be at 0");
        
        // Move 3: y=-1 (Wall collision)
        game.update_snake();
        assert!(game.game_over, "Should be game over at y=-1");
        let (_, head_y) = game.snake.head_position();
        assert_eq!(head_y, -1, "Head should be at -1");
    }

    #[test]
    fn test_obstacle_spawning() {
        let mut game = Game::new(20, 20);
        assert_eq!(game.obstacles.len(), 0);
        
        game.active_effects.clear(); // Ensure no effects interfere
        
        // Simulate eating 4 food, about to eat 5th
        game.food_eaten = 4; 
        
        // Snake moves Right by default. Head at (2,2). Next pos (3,2).
        let (hx, hy) = game.snake.head_position();
        game.food_x = hx + 1; // Place food ahead of snake
        game.food_y = hy;
        game.food_exists = true;
        
        game.waiting_time = 100.0; // Force update
        game.update(0.1); // Tick 1: Eats food. food_exists -> false.
        
        game.update(0.1); // Tick 2: !food_exists -> add_food() -> spawns obstacle.
        
        // Update runs update_snake -> moves to (hx+1, hy) -> check_eating -> add_food
        
        assert_eq!(game.food_eaten, 5, "Food should be eaten");
        assert_eq!(game.obstacles.len(), 1, "Should have 1 obstacle after eating 5th food");
    }

    #[test]
    fn test_multiplier_powerup() {
        let mut game = Game::new(20, 20);
        
        // Activate Multiplier manually
        game.active_effects.insert(PowerupType::Multiplier, 10.0);
        
        let initial_score = game.score;
        
        // Snake moves Right. Place food ahead.
        let (hx, hy) = game.snake.head_position();
        game.food_x = hx + 1;
        game.food_y = hy;
        game.food_exists = true;
        
        game.waiting_time = 100.0;
        
        game.update(0.1);
        
        // Base points = 1. Multiplier = 2x. Total = 2.
        assert_eq!(game.score, initial_score + 2, "Score should be doubled");
    }
}

pub enum CellType {
    Empty,
    SnakeHead,
    SnakeBody,
    Food,
    Obstacle,
    PowerupGhost,
    PowerupMultiplier,
}
