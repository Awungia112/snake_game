// Library root for snake game
// Re-exports all public modules

pub mod game;
pub mod game_logic; // Keep existing until fully migrated
pub mod multiplayer;
pub mod snake;
pub mod storage;
pub mod ui;

// Re-export commonly used types
pub use game_logic::Game;
pub use storage::{GameData, Difficulty};

