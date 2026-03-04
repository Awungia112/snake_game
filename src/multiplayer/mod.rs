// Multiplayer module - 2-player game state and input handling
pub mod game_state;
pub mod input;

pub use game_state::{MultiplayerGame, PlayerNumber};
pub use input::{PlayerInput, parse_multiplayer_input};
