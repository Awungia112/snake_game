// UI Module - all rendering logic
pub mod themes;
pub mod components;
pub mod game_view;
pub mod menu_view;
pub mod settings_view;
pub mod multiplayer_view;

pub use themes::THEMES;
pub use game_view::draw_game;
pub use menu_view::{draw_menu, draw_pause, draw_game_over};
pub use settings_view::draw_settings;
pub use multiplayer_view::{
    draw_multiplayer_setup,
    draw_multiplayer_game,
    draw_multiplayer_pause,
    draw_multiplayer_game_over,
};
