// Game module - core game logic and entities
pub mod entities;
pub mod settings;

// Re-export from game_logic
pub use crate::game_logic::{Game, GameDirection};

// Re-export entities
pub use entities::{Obstacle, Powerup, PowerupType, CellType, ActiveEffects};

// Re-export settings
pub use settings::GameSettings;
