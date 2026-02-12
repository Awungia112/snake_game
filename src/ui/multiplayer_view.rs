// Multiplayer UI views - setup screen, split-screen game, pause, and game over
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
    Frame,
};
use crate::multiplayer::{MultiplayerGame, PlayerNumber};
use crate::ui::themes::Theme;
use crate::ui::THEMES;

const COLOR_TEXT_PRIMARY: Color = Color::Rgb(220, 220, 220);
const COLOR_TEXT_SECONDARY: Color = Color::Rgb(150, 150, 150);
const COLOR_ACCENT: Color = Color::Rgb(100, 200, 255);

/// Draw the 2-player setup screen for theme selection
pub fn draw_multiplayer_setup(f: &mut Frame, p1_theme_idx: usize, p2_theme_idx: usize) {
    let size = f.size();
    
    // Main container
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_ACCENT))
        .title(" 2-PLAYER SETUP ");
    
    f.render_widget(Clear, size);
    f.render_widget(block, size);
    
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(1),  // Title
            Constraint::Length(2),  // Spacing
            Constraint::Length(10), // Player panels
            Constraint::Length(2),  // Spacing
            Constraint::Length(3),  // Instructions
        ])
        .split(size);
    
    // Title
    let title = Paragraph::new("Choose Your Themes")
        .style(Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    
    // Player panels side by side
    let player_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[2]);
    
    // Player 1 panel
    draw_player_theme_panel(f, player_chunks[0], 1, p1_theme_idx, &THEMES[p1_theme_idx]);
    
    // Player 2 panel
    draw_player_theme_panel(f, player_chunks[1], 2, p2_theme_idx, &THEMES[p2_theme_idx]);
    
    // Instructions
    let instructions = vec![
        Line::from(Span::styled("Player 1: ←/→ to change theme", Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(Span::styled("Player 2: A/D to change theme", Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(Span::styled("Press Enter to Start | Q to Menu", Style::default().fg(COLOR_ACCENT).add_modifier(Modifier::BOLD))),
    ];
    let instructions_para = Paragraph::new(instructions)
        .alignment(Alignment::Center);
    f.render_widget(instructions_para, chunks[4]);
}

fn draw_player_theme_panel(f: &mut Frame, area: Rect, player_num: u8, _theme_idx: usize, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border))
        .title(format!(" Player {} ", player_num));
    
    let inner = block.inner(area);
    f.render_widget(block, area);
    
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("▶ Theme: {}", theme.name),
            Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Snake: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled("◉ ▪▪▪", Style::default().fg(theme.snake_head)),
        ]),
        Line::from(vec![
            Span::styled("  Border: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled("━━━", Style::default().fg(theme.border)),
        ]),
    ];
    
    let para = Paragraph::new(lines)
        .alignment(Alignment::Center);
    f.render_widget(para, inner);
}

/// Draw the split-screen multiplayer game
pub fn draw_multiplayer_game(f: &mut Frame, mp_game: &MultiplayerGame) {
    let size = f.size();
    
    // Split screen vertically
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(size);
    
    let p1_theme = &THEMES[mp_game.player1_theme_index];
    let p2_theme = &THEMES[mp_game.player2_theme_index];
    
    // Draw player 1 area
    draw_player_game_area(f, main_chunks[0], &mp_game.player1, 1, p1_theme);
    
    // Draw player 2 area
    draw_player_game_area(f, main_chunks[1], &mp_game.player2, 2, p2_theme);
    
    // Draw session footer
    draw_session_footer(f, size, mp_game);
}

fn draw_player_game_area(f: &mut Frame, area: Rect, game: &crate::game_logic::Game, _player_num: u8, _theme: &Theme) {
    // Use the existing draw_game function but in a constrained area
    use crate::ui::game_view::draw_game;
    draw_game(f, game, area);
}


fn draw_session_footer(f: &mut Frame, area: Rect, mp_game: &MultiplayerGame) {
    let footer_area = Rect {
        x: area.x,
        y: area.y + area.height - 3,
        width: area.width,
        height: 3,
    };
    
    let text = format!(
        " Session: P1: {} wins | P2: {} wins | P Pause | Q Menu ",
        mp_game.player1_wins,
        mp_game.player2_wins
    );
    
    let footer = Paragraph::new(text)
        .style(Style::default().fg(COLOR_TEXT_SECONDARY))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(COLOR_ACCENT)));
    
    f.render_widget(footer, footer_area);
}

/// Draw multiplayer pause screen
pub fn draw_multiplayer_pause(f: &mut Frame, mp_game: &MultiplayerGame) {
    let size = f.size();
    
    // Create centered popup
    let popup_area = centered_rect(40, 30, size);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_ACCENT))
        .title(" PAUSED ");
    
    f.render_widget(Clear, popup_area);
    f.render_widget(block.clone(), popup_area);
    
    let inner = block.inner(popup_area);
    
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("Game Paused", Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled(format!("P1 Wins: {}", mp_game.player1_wins), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(Span::styled(format!("P2 Wins: {}", mp_game.player2_wins), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(""),
        Line::from(Span::styled("P - Resume", Style::default().fg(COLOR_ACCENT))),
        Line::from(Span::styled("Q - Quit to Menu", Style::default().fg(COLOR_ACCENT))),
    ];
    
    let para = Paragraph::new(lines)
        .alignment(Alignment::Center);
    f.render_widget(para, inner);
}

/// Draw multiplayer game over screen
pub fn draw_multiplayer_game_over(f: &mut Frame, mp_game: &MultiplayerGame) {
    let size = f.size();
    
    // Create centered popup
    let popup_area = centered_rect(50, 40, size);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_ACCENT))
        .title(" GAME OVER ");
    
    f.render_widget(Clear, popup_area);
    f.render_widget(block.clone(), popup_area);
    
    let inner = block.inner(popup_area);
    
    let winner_text = match mp_game.winner {
        Some(PlayerNumber::One) => "🏆 Player 1 Wins! 🏆",
        Some(PlayerNumber::Two) => "🏆 Player 2 Wins! 🏆",
        None => "It's a Draw!",
    };
    
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(winner_text, Style::default().fg(COLOR_ACCENT).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled(format!("P1 Score: {}", mp_game.player1.score()), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(Span::styled(format!("P2 Score: {}", mp_game.player2.score()), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(""),
        Line::from(Span::styled("Session Wins", Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled(format!("Player 1: {}", mp_game.player1_wins), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(Span::styled(format!("Player 2: {}", mp_game.player2_wins), Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(""),
        Line::from(Span::styled("R - Rematch", Style::default().fg(COLOR_ACCENT))),
        Line::from(Span::styled("Q - Quit to Menu", Style::default().fg(COLOR_ACCENT))),
    ];
    
    let para = Paragraph::new(lines)
        .alignment(Alignment::Center);
    f.render_widget(para, inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
