// Menu views rendering
// Handles drawing menu, pause, and game over screens

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
    Frame,
};

use crate::storage::Difficulty;
use super::themes::{Theme, COLOR_SCORE, COLOR_TEXT_SECONDARY};

pub fn draw_menu(f: &mut Frame, selected: usize, high_score: u32, difficulty: &Difficulty, theme: &Theme) {
    let size = f.size();
    
    let panel_width = 50;
    let panel_height = 22;
    let panel_area = Rect {
        x: (size.width.saturating_sub(panel_width)) / 2,
        y: (size.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    // Title
    let title_lines = vec![
        Line::from(Span::styled("🐍 SNAKE", Style::default().fg(theme.border).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("Rust Edition", Style::default().fg(COLOR_TEXT_SECONDARY))),
        Line::from(""),
        Line::from(vec![
            Span::styled("★ Best: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled(format!("{}", high_score), Style::default().fg(theme.combo).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled(difficulty.name(), Style::default().fg(theme.snake_head)),
        ]),
        Line::from(vec![
            Span::styled("Theme: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
            Span::styled(theme.name, Style::default().fg(theme.multiplier)),
        ]),
        Line::from(""),
    ];
    
    let title_widget = Paragraph::new(title_lines).alignment(Alignment::Center);
    let title_area = Rect { x: inner.x, y: inner.y, width: inner.width, height: 8 };
    f.render_widget(title_widget, title_area);
    
    // Menu options
    let options = ["▶ Start Game", "  Settings", "  Quit"];
    let mut menu_lines = vec![];
    
    for (i, option) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT_SECONDARY)
        };
        menu_lines.push(Line::from(Span::styled(*option, style)));
        menu_lines.push(Line::from(""));
    }
    
    let menu_widget = Paragraph::new(menu_lines).alignment(Alignment::Center);
    let menu_area = Rect { x: inner.x, y: inner.y + 9, width: inner.width, height: 8 };
    f.render_widget(menu_widget, menu_area);
    
    // Footer
    let footer_text = "↑↓ Nav • Enter • T Diff • H Theme • Q Quit";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(COLOR_TEXT_SECONDARY))
        .alignment(Alignment::Center);
    let footer_area = Rect { x: inner.x, y: inner.y + inner.height - 2, width: inner.width, height: 1 };
    f.render_widget(footer, footer_area);
}

pub fn draw_pause(f: &mut Frame, _difficulty: &Difficulty, selected: usize, theme: &Theme) {
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
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    let options = [
        ("P - Resume", "Resume"),
        ("Q - Quit to Menu", "Quit to Menu"),
    ];
    
    let mut lines = vec![
        Line::from(Span::styled("⏸  PAUSED", Style::default().fg(theme.border).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    for (i, (text, _)) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COLOR_TEXT_SECONDARY)
        };
        let prefix = if i == selected { "▶ " } else { "  " };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, text), style)));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!("Theme: {} (T)", theme.name), Style::default().fg(theme.snake_head))));
    
    let widget = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(widget, inner);
}

pub fn draw_game_over(f: &mut Frame, score: u32, high_score: u32, is_new_high: bool, selected: usize, theme: &Theme) {
    let size = f.size();
    
    let panel_width = 40;
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
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    let mut lines = vec![
        Line::from(Span::styled("💀 GAME OVER", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    if is_new_high {
        lines.push(Line::from(Span::styled("NEW HIGH SCORE! 🎉", Style::default().fg(theme.combo).add_modifier(Modifier::BOLD))));
        lines.push(Line::from(""));
    }
    
    lines.push(Line::from(vec![
        Span::styled("Score: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::styled(format!("{}", score), Style::default().fg(COLOR_SCORE).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Best: ", Style::default().fg(COLOR_TEXT_SECONDARY)),
        Span::styled(format!("{}", high_score), Style::default().fg(theme.border).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));
    
    let options = ["R - Restart", "Q - Quit to Menu"];
    for (i, option) in options.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
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
