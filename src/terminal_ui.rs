use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Clear},
    Frame,
};
use crate::game_logic::{Game, CellType};
use crate::storage::Difficulty;

// Modern color palette
const COLOR_BG: Color = Color::Rgb(23, 28, 36);
const COLOR_BORDER: Color = Color::Rgb(51, 191, 224);
const COLOR_SNAKE_HEAD: Color = Color::Rgb(102, 224, 153);
const COLOR_SNAKE_BODY: Color = Color::Rgb(89, 209, 140);
const COLOR_FOOD: Color = Color::Rgb(250, 107, 133);
const COLOR_SCORE: Color = Color::Rgb(242, 199, 89);
const COLOR_TEXT_PRIMARY: Color = Color::Rgb(242, 245, 247);
const COLOR_TEXT_SECONDARY: Color = Color::Rgb(179, 186, 194);

pub fn draw_game(f: &mut Frame, game: &Game) {
    let size = f.size();
    
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Game area
            Constraint::Length(3),  // Footer
        ])
        .split(size);
    
    // Header with score
    draw_header(f, chunks[0], game);
    
    // Game area
    draw_game_area(f, chunks[1], game);
    
    // Footer with controls
    draw_footer(f, chunks[2], game);
}

fn draw_header(f: &mut Frame, area: Rect, game: &Game) {
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
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(COLOR_BORDER)));
    f.render_widget(score, header_chunks[0]);
    
    // Title
    let title = Paragraph::new("🐍 SNAKE")
        .style(Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(COLOR_BORDER)));
    f.render_widget(title, header_chunks[1]);
    
    // High Score
    let high_score_text = format!("Best: {}", game.high_score());
    let high_score = Paragraph::new(high_score_text)
        .style(Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(COLOR_BORDER)));
    f.render_widget(high_score, header_chunks[2]);
}

fn draw_game_area(f: &mut Frame, area: Rect, game: &Game) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_BORDER))
        .style(Style::default().bg(COLOR_BG));
    
    let inner = block.inner(area);
    f.render_widget(block, area);
    
    // Draw game content - use full available area, no centering
    let game_width = game.width() as u16;
    let game_height = game.height() as u16;
    
    // Use the full inner area without centering offset
    // Each cell is 2 characters wide for consistent horizontal/vertical movement speed
    let game_area = Rect {
        x: inner.x,
        y: inner.y,
        width: (game_width * 2).min(inner.width),
        height: game_height.min(inner.height),
    };
    
    // Draw snake and food with better visuals
    let mut lines = vec![];
    for y in 0..game_height.min(inner.height) {
        let mut line_spans = vec![];
        for x in 0..game_width {
            let cell = game.get_cell(x as i32, y as i32);
            let (char, color) = match cell {
                CellType::Empty => ("  ", COLOR_BG),
                CellType::SnakeHead => ("◉ ", COLOR_SNAKE_HEAD),
                CellType::SnakeBody => ("▪ ", COLOR_SNAKE_BODY),
                CellType::Food => ("◆ ", COLOR_FOOD),
            };
            line_spans.push(Span::styled(char, Style::default().fg(color).bg(COLOR_BG)));
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
}

fn draw_footer(f: &mut Frame, area: Rect, game: &Game) {
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
        Span::styled("Q", Style::default().fg(COLOR_TEXT_PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::raw(" | "),
        Span::styled(format!("{}", game.difficulty().name()), Style::default().fg(diff_color).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(format!("🍎{}", game.food_eaten()), Style::default().fg(COLOR_FOOD)),
        Span::raw(" | "),
        Span::styled(format!("⏱ {}:{:02}", mins, secs), Style::default().fg(COLOR_TEXT_SECONDARY)),
    ];
    
    let footer = Paragraph::new(Line::from(footer_text))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(COLOR_BORDER)));
    
    f.render_widget(footer, area);
}


pub fn draw_menu(f: &mut Frame, selected: usize, high_score: u32, difficulty: &Difficulty) {
    let size = f.size();
    
    // Center panel
    let panel_width = 50;
    let panel_height = 20;
    let panel_area = Rect {
        x: (size.width.saturating_sub(panel_width)) / 2,
        y: (size.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_BORDER))
        .style(Style::default().bg(COLOR_BG));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    // Title
    let title_lines = vec![
        Line::from(Span::styled("🐍 SNAKE", Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("Rust Edition", Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(""),
        Line::from(vec![
            Span::styled("★ Best: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled(format!("{}", high_score), Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled(difficulty.name(), Style::default().fg(COLOR_SCORE)),
        ]),
        Line::from(""),
    ];
    
    let title_widget = Paragraph::new(title_lines).alignment(Alignment::Center);
    let title_area = Rect { x: inner.x, y: inner.y, width: inner.width, height: 7 };
    f.render_widget(title_widget, title_area);
    
    // Menu options
    let options = ["▶ Start Game", "  How to Play", "  Quit"];
    let mut menu_lines = vec![];
    
    for (i, option) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT_SECONDARY)
        };
        menu_lines.push(Line::from(Span::styled(*option, style)));
        menu_lines.push(Line::from(""));
    }
    
    let menu_widget = Paragraph::new(menu_lines).alignment(Alignment::Center);
    let menu_area = Rect { x: inner.x, y: inner.y + 8, width: inner.width, height: 8 };
    f.render_widget(menu_widget, menu_area);
    
    // Footer
    let footer = Paragraph::new("↑↓ Navigate  •  Enter Select  •  T Difficulty  •  Q Quit")
        .style(Style::default().fg(COLOR_TEXT_SECONDARY))
        .alignment(Alignment::Center);
    let footer_area = Rect { x: inner.x, y: inner.y + inner.height - 2, width: inner.width, height: 1 };
    f.render_widget(footer, footer_area);
}

pub fn draw_pause(f: &mut Frame, _difficulty: &Difficulty, selected: usize) {
    let size = f.size();
    
    let panel_width = 40;
    let panel_height = 10;
    let panel_area = Rect {
        x: (size.width.saturating_sub(panel_width)) / 2,
        y: (size.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(COLOR_BORDER))
        .style(Style::default().bg(COLOR_BG));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    let options = [
        ("P - Resume", "Resume"),
        ("Q - Quit to Menu", "Quit to Menu"),
    ];
    
    let mut lines = vec![
        Line::from(Span::styled("⏸  PAUSED", Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    for (i, (text, _)) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT_SECONDARY)
        };
        let prefix = if i == selected { "▶ " } else { "  " };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, text), style)));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("↑↓ Navigate  •  Enter Select", Style::default().fg(COLOR_TEXT_SECONDARY))));
    
    let widget = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(widget, inner);
}

pub fn draw_game_over(f: &mut Frame, score: u32, high_score: u32, is_new_high: bool, selected: usize) {
    let size = f.size();
    
    let panel_width = 45;
    let panel_height = 14;
    let panel_area = Rect {
        x: (size.width.saturating_sub(panel_width)) / 2,
        y: (size.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(COLOR_BG));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    let mut lines = vec![
        Line::from(Span::styled("GAME OVER", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    if is_new_high {
        lines.push(Line::from(Span::styled("★ NEW RECORD! ★", Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD))));
        lines.push(Line::from(""));
    }
    
    lines.push(Line::from(vec![
        Span::styled("Score: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::styled(format!("{}", score), Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Best: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::styled(format!("{}", high_score), Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    
    let options = ["R - Restart", "Q - Quit to Menu"];
    for (i, option) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(COLOR_BORDER).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT_SECONDARY)
        };
        let prefix = if i == selected { "▶ " } else { "  " };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, option), style)));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("↑↓ Navigate  •  Enter Select", Style::default().fg(COLOR_TEXT_SECONDARY))));
    
    let widget = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(widget, inner);
}
