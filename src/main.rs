use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

// Minimum terminal size required
const MIN_WIDTH: u16 = 70;
const MIN_HEIGHT: u16 = 28;

fn calculate_game_dimensions(term_width: u16, term_height: u16) -> (i32, i32) {
    let usable_width = term_width.saturating_sub(2);
    let usable_height = term_height.saturating_sub(8);
    ((usable_width / 2) as i32, usable_height as i32)
}

mod game_logic;
mod snake;
mod storage;
mod ui;

use game_logic::{Game, GameDirection};
use storage::GameData;
use ui::{draw_game, draw_menu, draw_pause, draw_game_over, draw_settings, THEMES};

enum AppState {
    Menu,
    Playing,
    Paused,
    GameOver,
    Settings,
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let _game_data = GameData::load();
    let size = terminal.size()?;
    let (game_width, game_height) = calculate_game_dimensions(size.width, size.height);
    let mut game = Game::new(game_width, game_height);
    let mut app_state = AppState::Menu;
    let mut menu_selected = 0;
    let menu_options = 3;
    let mut settings_selected = 0;
    let settings_options = 5; // Theme, Preset, Obstacle, Powerup, Back
    let mut pause_selected = 0;
    let pause_options = 2;
    let mut game_over_selected = 0;
    let game_over_options = 2;

    let mut last_update = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        // Check terminal size and handle resize
        let size = terminal.size()?;
        
        // Update game dimensions if terminal was resized during gameplay
        let (current_game_width, current_game_height) = calculate_game_dimensions(size.width, size.height);
        if matches!(app_state, AppState::Playing | AppState::Paused) {
            if current_game_width != game.width() || current_game_height != game.height() {
                if size.width >= MIN_WIDTH && size.height >= MIN_HEIGHT {
                    game.resize(current_game_width, current_game_height);
                }
            }
        }
        
        if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
            terminal.draw(|f| {
                use ratatui::{
                    layout::Alignment,
                    style::{Color, Modifier, Style},
                    text::Line,
                    widgets::{Block, Borders, Paragraph},
                };
                let warning = Paragraph::new(vec![
                    Line::from("⚠ Terminal Too Small!"),
                    Line::from(""),
                    Line::from(format!("Current: {}x{}", size.width, size.height)),
                    Line::from(format!("Required: {}x{}", MIN_WIDTH, MIN_HEIGHT)),
                    Line::from(""),
                    Line::from("Please resize your terminal"),
                    Line::from("or press Q to quit"),
                ])
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Warning"));
                f.render_widget(warning, f.size());
            })?;
            
            // Wait for resize or quit
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        break;
                    }
                }
            }
            continue;
        }

        terminal.draw(|f| {
            let current_theme = &THEMES[game.theme_index];
            match app_state {
                AppState::Menu => {
                    draw_menu(f, menu_selected, game.high_score(), game.difficulty(), current_theme);
                }
                AppState::Playing => {
                    draw_game(f, &game);
                }
                AppState::Paused => {
                    draw_game(f, &game);
                    draw_pause(f, game.difficulty(), pause_selected, current_theme);
                }
                AppState::GameOver => {
                    draw_game(f, &game);
                    let is_new_high = game.score() == game.high_score() && game.score() > 0;
                    draw_game_over(f, game.score(), game.high_score(), is_new_high, game_over_selected, current_theme);
                }
                AppState::Settings => {
                    draw_settings(f, &game, settings_selected, current_theme);
                }
            }
        })?;

        // Handle input
        let timeout = tick_rate.saturating_sub(last_update.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app_state {
                    AppState::Menu => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => break,
                            KeyCode::Char('t') | KeyCode::Char('T') => {
                                game.change_difficulty();
                            }
                            KeyCode::Char('h') | KeyCode::Char('H') => {
                                game.cycle_theme();
                            }
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                if menu_selected > 0 {
                                    menu_selected -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                if menu_selected < menu_options - 1 {
                                    menu_selected += 1;
                                }
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                match menu_selected {
                                    0 => {
                                        let size = terminal.size()?;
                                        let (w, h) = calculate_game_dimensions(size.width, size.height);
                                        game = Game::new(w, h);
                                        app_state = AppState::Playing;
                                    }
                                    1 => {
                                        app_state = AppState::Settings;
                                    }
                                    2 => {
                                        // TODO: Show instructions
                                    }
                                    3 => break,
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    AppState::Playing => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    break;
                                } else {
                                    app_state = AppState::Menu;
                                }
                            }
                            KeyCode::Char('p') | KeyCode::Char('P') => {
                                app_state = AppState::Paused;
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                let size = terminal.size()?;
                                let (w, h) = calculate_game_dimensions(size.width, size.height);
                                game = Game::new(w, h);
                            }
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                game.handle_direction(GameDirection::Up);
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                game.handle_direction(GameDirection::Down);
                            }
                            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                                game.handle_direction(GameDirection::Left);
                            }
                            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                                game.handle_direction(GameDirection::Right);
                            }
                            KeyCode::Char('h') | KeyCode::Char('H') => {
                                game.cycle_theme();
                            }
                            KeyCode::Char('o') | KeyCode::Char('O') => {
                                game.adjust_spawn_rates();
                            }
                            _ => {}
                        }

                        if game.is_game_over() {
                            app_state = AppState::GameOver;
                        }
                    }
                    AppState::Paused => {
                        match key.code {
                            KeyCode::Char('h') | KeyCode::Char('H') => {
                                game.cycle_theme();
                            }
                            KeyCode::Char('o') | KeyCode::Char('O') => {
                                game.adjust_spawn_rates();
                            }
                            KeyCode::Char('t') | KeyCode::Char('T') => {
                                game.cycle_theme();
                            }
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                if pause_selected > 0 {
                                    pause_selected -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                if pause_selected < pause_options - 1 {
                                    pause_selected += 1;
                                }
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                match pause_selected {
                                    0 => app_state = AppState::Playing,
                                    1 => app_state = AppState::Menu,
                                    _ => {}
                                }
                            }
                            KeyCode::Char('p') | KeyCode::Char('P') => {
                                app_state = AppState::Playing;
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                app_state = AppState::Menu;
                            }
                            _ => {}
                        }
                    }
                    AppState::Settings => {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                if settings_selected > 0 {
                                    settings_selected -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                if settings_selected < settings_options - 1 {
                                    settings_selected += 1;
                                }
                            }
                            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                                match settings_selected {
                                    0 => game.cycle_theme(), // Theme (cycles backward, but we'll use same function)
                                    2 => game.decrease_obstacle_interval(),
                                    3 => game.decrease_powerup_interval(),
                                    _ => {}
                                }
                            }
                            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                                match settings_selected {
                                    0 => game.cycle_theme(), // Theme
                                    2 => game.increase_obstacle_interval(),
                                    3 => game.increase_powerup_interval(),
                                    _ => {}
                                }
                            }
                            KeyCode::Enter | KeyCode::Char('o') | KeyCode::Char('O') => {
                                if settings_selected == 1 {
                                    game.adjust_spawn_rates(); // Preset
                                } else if settings_selected == 4 {
                                    app_state = AppState::Menu; // Back
                                }
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                app_state = AppState::Menu;
                            }
                            _ => {}
                        }
                    }
                    AppState::GameOver => {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                if game_over_selected > 0 {
                                    game_over_selected -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                if game_over_selected < game_over_options - 1 {
                                    game_over_selected += 1;
                                }
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                match game_over_selected {
                                    0 => {
                                        let size = terminal.size()?;
                                        let (w, h) = calculate_game_dimensions(size.width, size.height);
                                        game = Game::new(w, h);
                                        app_state = AppState::Playing;
                                    }
                                    1 => app_state = AppState::Menu,
                                    _ => {}
                                }
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                let size = terminal.size()?;
                                let (w, h) = calculate_game_dimensions(size.width, size.height);
                                game = Game::new(w, h);
                                app_state = AppState::Playing;
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                app_state = AppState::Menu;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Update game state
        if last_update.elapsed() >= tick_rate {
            if let AppState::Playing = app_state {
                game.update(tick_rate.as_secs_f64());
                
                // Audio feedback - terminal bell on eating food
                if game.should_beep() {
                    print!("\x07"); // Bell character
                }
                
                if game.is_game_over() {
                    app_state = AppState::GameOver;
                }
            }
            last_update = Instant::now();
        }
    }

    Ok(())
}
