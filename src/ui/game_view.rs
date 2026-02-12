// Game view rendering
// Handles drawing the main game screen with header, game area, and footer

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph},
    Frame,
};

use crate::game_logic::{CellType, Game, PowerupType};
use crate::storage::Difficulty;
use super::themes::{Theme, THEMES, COLOR_SCORE, COLOR_TEXT_PRIMARY, COLOR_TEXT_SECONDARY};

pub fn draw_game(frame: &mut Frame, game: &Game, area: Rect) {
    let current_theme = &THEMES[game.theme_index];
    
    // Background
    frame.render_widget(Block::default().style(Style::default().bg(current_theme.bg)), area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Game Area
            Constraint::Length(3), // Footer
        ])
        .split(area);

    draw_header(frame, chunks[0], game, current_theme);
    draw_game_area(frame, chunks[1], game, current_theme);
    draw_footer(frame, chunks[2], game, current_theme);
}

fn draw_header(f: &mut Frame, area: Rect, game: &Game, theme: &Theme) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);
    
    // Score
    let score_text = format!("Score: {}", game.score());
    let score = Paragraph::new(score_text)
        .style(Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border)));
    f.render_widget(score, header_chunks[0]);
    
    // Status / Title
    let mut status_text = String::from("🐍 SNAKE");
    
    // Append active effects
    if let Some(&duration) = game.active_effects().get(&PowerupType::Ghost) {
        status_text.push_str(&format!(" | GHOST: {:.1}s", duration));
    }
    if let Some(&duration) = game.active_effects().get(&PowerupType::Multiplier) {
        status_text.push_str(&format!(" | 2X: {:.1}s", duration));
    }

    let title = Paragraph::new(status_text)
        .style(Style::default().fg(theme.border).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border)));
    f.render_widget(title, header_chunks[1]);
    
    // High Score
    let high_score_text = format!("Best: {}", game.high_score());
    let high_score = Paragraph::new(high_score_text)
        .style(Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border)));
    f.render_widget(high_score, header_chunks[2]);
}

fn draw_game_area(f: &mut Frame, area: Rect, game: &Game, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));
    
    let inner = block.inner(area);
    f.render_widget(block, area);
    
    // Draw game content
    let game_width = game.width() as u16;
    let game_height = game.height() as u16;
    
    let game_area = Rect {
        x: inner.x,
        y: inner.y,
        width: (game_width * 2).min(inner.width),
        height: game_height.min(inner.height),
    };
    
    // Draw snake and food
    let mut lines = vec![];
    for y in 0..game_height.min(inner.height) {
        let mut line_spans = vec![];
        for x in 0..game_width {
            let cell = game.get_cell(x as i32, y as i32);
            let (symbol, color) = match cell {
                CellType::Empty => ("  ", theme.bg),
                CellType::SnakeHead => {
                    let mut color = if game.combo() > 1 { theme.combo } else { theme.snake_head };
                    if game.is_ghost_active() { color = theme.ghost; }
                    ("◉ ", color)
                },
                CellType::SnakeBody => {
                    let mut color = if game.combo() > 1 { theme.combo } else { theme.snake_body };
                    if game.is_ghost_active() { color = theme.ghost; }
                    ("▪ ", color)
                },
                CellType::Food => ("🍎", theme.food),
                CellType::Obstacle => ("▓▓", theme.obstacle),
                CellType::PowerupGhost => ("👻", theme.ghost),
                CellType::PowerupMultiplier => ("2x", theme.multiplier),
            };
            line_spans.push(Span::styled(symbol, Style::default().fg(color).bg(theme.bg)));
        }
        lines.push(Line::from(line_spans));
    }
    
    let game_content = Paragraph::new(lines);
    f.render_widget(game_content, game_area);
    
    // Draw combo if active
    if game.combo() > 1 {
        let combo_text = format!("x{} COMBO!", game.combo());
        let combo_widget = Paragraph::new(combo_text)
            .style(Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        
        let combo_area = Rect {
            x: inner.x + inner.width / 4,
            y: inner.y + 2,
            width: inner.width / 2,
            height: 1,
        };
        f.render_widget(combo_widget, combo_area);
    }
    
    // Draw score popup if active
    if let Some((points, time)) = game.score_popup() {
        if time > 0.0 {
            let popup_text = format!("+{}", points);
            let alpha = (time / 1.0 * 255.0) as u8;
            let popup_color = Color::Rgb(242, 199, alpha);
            let popup_widget = Paragraph::new(popup_text)
                .style(Style::default().fg(popup_color).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            
            let popup_area = Rect {
                x: inner.x + inner.width / 3,
                y: inner.y + 4,
                width: inner.width / 3,
                height: 1,
            };
            f.render_widget(popup_widget, popup_area);
        }
    }
    
    // Draw collision head if game over
    if game.is_game_over() {
        let (head_x, head_y) = game.head_position();
        let width = game.width();
        let height = game.height();
        
        let should_draw_crash = head_x < 0 || head_x >= width || head_y < 0 || head_y >= height;
        
        if should_draw_crash {
            let screen_x = inner.x as i32 + head_x * 2;
            let screen_y = inner.y as i32 + head_y;
            
            let term_width = f.size().width as i32;
            let term_height = f.size().height as i32;
            
            if screen_x >= 0 && screen_y >= 0 && screen_x + 1 < term_width && screen_y < term_height {
                let crash_area = Rect {
                    x: screen_x as u16,
                    y: screen_y as u16,
                    width: 2,
                    height: 1,
                };
                
                let crash_color = Color::Red;
                let crash_head = Paragraph::new("X ")
                    .style(Style::default().fg(crash_color).bg(theme.bg).add_modifier(Modifier::BOLD));
                    
                f.render_widget(crash_head, crash_area);
            }
        }
    }
}

fn draw_footer(f: &mut Frame, area: Rect, game: &Game, theme: &Theme) {
    let diff_color = match game.difficulty() {
        Difficulty::Easy => Color::Green,
        Difficulty::Medium => COLOR_SCORE,
        Difficulty::Hard => Color::Red,
    };
    
    let mins = (game.play_time() / 60.0) as u32;
    let secs = (game.play_time() % 60.0) as u32;
    
    let footer_text = vec![
        Span::styled("↑↓←→/WASD", Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled("P", Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled(" Pause", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::raw(" | "),
        Span::styled(format!("{:?}", game.difficulty()), Style::default().fg(diff_color).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(format!("{}:{:02}", mins, secs), Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::raw(" | "),
        Span::styled(format!("Obs: {:.0}s", game.settings.obstacle_interval), Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::raw(" | "),
        Span::styled(format!("Pow: {:.0}s", game.settings.powerup_interval), Style::default().fg(COLOR_TEXT_SECONDARY)),
    ];
    
    let footer = Paragraph::new(Line::from(footer_text))
        .style(Style::default().fg(theme.border))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme.border)));
    
    f.render_widget(footer, area);
}
