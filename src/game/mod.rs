// Game Module
// Core game logic and state management
// TODO: Split game_logic.rs into submodules

// For now, re-export from game_logic until we complete the migration
pub use crate::game_logic::{
    Game, GameSettings, Obstacle, Powerup, PowerupType, CellType
};
pub use crate::storage::Difficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}

