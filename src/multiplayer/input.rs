// Input handling for 2-player mode
use crossterm::event::KeyCode;
use crate::game::GameDirection;

pub enum PlayerInput {
    Player1(GameDirection),
    Player2(GameDirection),
    Pause,
    Quit,
}

pub fn parse_multiplayer_input(key: KeyCode) -> Option<PlayerInput> {
    match key {
        // Player 1 - Arrow keys
        KeyCode::Up => Some(PlayerInput::Player1(GameDirection::Up)),
        KeyCode::Down => Some(PlayerInput::Player1(GameDirection::Down)),
        KeyCode::Left => Some(PlayerInput::Player1(GameDirection::Left)),
        KeyCode::Right => Some(PlayerInput::Player1(GameDirection::Right)),

        // Player 2 - WASD
        KeyCode::Char('w') | KeyCode::Char('W') => Some(PlayerInput::Player2(GameDirection::Up)),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(PlayerInput::Player2(GameDirection::Down)),
        KeyCode::Char('a') | KeyCode::Char('A') => Some(PlayerInput::Player2(GameDirection::Left)),
        KeyCode::Char('d') | KeyCode::Char('D') => Some(PlayerInput::Player2(GameDirection::Right)),

        // Shared controls
        KeyCode::Char('p') | KeyCode::Char('P') => Some(PlayerInput::Pause),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(PlayerInput::Quit),

        _ => None,
    }
}
