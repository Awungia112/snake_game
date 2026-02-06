// UI Module
// Handles all terminal UI rendering

pub mod components;
pub mod game_view;
pub mod menu_view;
pub mod settings_view;
pub mod themes;

// Re-export main drawing functions
pub use game_view::draw_game;
pub use menu_view::{draw_game_over, draw_menu, draw_pause};
pub use settings_view::draw_settings;
pub use themes::THEMES;
