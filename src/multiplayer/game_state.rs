// Multiplayer game state management
use crate::game::{Game, GameDirection};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerNumber {
    One,
    Two,
}

pub struct MultiplayerGame {
    pub player1: Game,
    pub player2: Game,
    pub player1_theme_index: usize,
    pub player2_theme_index: usize,
    pub player1_wins: u32,
    pub player2_wins: u32,
    pub game_over: bool,
    pub winner: Option<PlayerNumber>,
    pub paused: bool,
}

impl MultiplayerGame {
    pub fn new(width: i32, height: i32, p1_theme: usize, p2_theme: usize) -> Self {
        MultiplayerGame {
            player1: Game::new(width, height),
            player2: Game::new(width, height),
            player1_theme_index: p1_theme,
            player2_theme_index: p2_theme,
            player1_wins: 0,
            player2_wins: 0,
            game_over: false,
            winner: None,
            paused: false,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.paused || self.game_over {
            return;
        }

        // Update both games
        self.player1.update(delta_time);
        self.player2.update(delta_time);

        // Check for game over
        self.check_game_over();
    }

    fn check_game_over(&mut self) {
        let p1_dead = self.player1.is_game_over();
        let p2_dead = self.player2.is_game_over();

        if p1_dead && !p2_dead {
            self.winner = Some(PlayerNumber::Two);
            self.player2_wins += 1;
            self.game_over = true;
        } else if p2_dead && !p1_dead {
            self.winner = Some(PlayerNumber::One);
            self.player1_wins += 1;
            self.game_over = true;
        } else if p1_dead && p2_dead {
            // Both died - it's a draw
            self.winner = None;
            self.game_over = true;
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.player1.resize(width, height);
        self.player2.resize(width, height);
    }

    pub fn handle_player1_input(&mut self, direction: GameDirection) {
        if !self.paused && !self.game_over {
            self.player1.change_direction(direction);
        }
    }

    pub fn handle_player2_input(&mut self, direction: GameDirection) {
        if !self.paused && !self.game_over {
            self.player2.change_direction(direction);
        }
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
            self.player1.toggle_pause();
            self.player2.toggle_pause();
        }
    }

    pub fn reset_round(&mut self, width: i32, height: i32) {
        // Keep the same themes and session wins
        let p1_theme = self.player1_theme_index;
        let p2_theme = self.player2_theme_index;
        let p1_wins = self.player1_wins;
        let p2_wins = self.player2_wins;

        *self = MultiplayerGame::new(width, height, p1_theme, p2_theme);
        self.player1_wins = p1_wins;
        self.player2_wins = p2_wins;
    }
}
