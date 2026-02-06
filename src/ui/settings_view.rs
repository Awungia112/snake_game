// Settings view rendering
// Handles drawing the settings/configuration screen

use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
    Frame,
};

use crate::game_logic::Game;
use super::themes::{Theme, COLOR_TEXT_SECONDARY};

pub fn draw_settings(f: &mut Frame, game: &Game, selected: usize, theme: &Theme) {
    let size = f.size();
    
    let panel_width = 60;
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
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));
    
    let inner = block.inner(panel_area);
    f.render_widget(Clear, panel_area);
    f.render_widget(block, panel_area);
    
    let mut lines = vec![
        Line::from(Span::styled("⚙  SETTINGS", Style::default().fg(theme.border).add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];
    
    // Theme
    let theme_style = if selected == 0 {
        Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_TEXT_SECONDARY)
    };
    let prefix = if selected == 0 { "▶ " } else { "  " };
    lines.push(Line::from(Span::styled(
        format!("{}Theme: {} (←/→)", prefix, theme.name),
        theme_style
    )));
    
    lines.push(Line::from(""));
    
    // Preset
    let preset_name = game.get_spawn_preset_name();
    let preset_style = if selected == 1 {
        Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_TEXT_SECONDARY)
    };
    let prefix = if selected == 1 { "▶ " } else { "  " };
    lines.push(Line::from(Span::styled(
        format!("{}Preset: {} (Enter/O)", prefix, preset_name),
        preset_style
    )));
    
    lines.push(Line::from(""));
    
    // Obstacle Interval
    let obs_style = if selected == 2 {
        Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_TEXT_SECONDARY)
    };
    let prefix = if selected == 2 { "▶ " } else { "  " };
    lines.push(Line::from(Span::styled(
        format!("{}Obstacle Spawn: {:.1}s (←/→)", prefix, game.settings.obstacle_interval),
        obs_style
    )));
    
    // Powerup Interval
    let pow_style = if selected == 3 {
        Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_TEXT_SECONDARY)
    };
    let prefix = if selected == 3 { "▶ " } else { "  " };
    lines.push(Line::from(Span::styled(
        format!("{}Powerup Spawn: {:.1}s (←/→)", prefix, game.settings.powerup_interval),
        pow_style
    )));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("0s = Disabled", Style::default().fg(COLOR_TEXT_SECONDARY))));
    lines.push(Line::from(""));
    
    // Back button
    let back_style = if selected == 4 {
        Style::default().fg(theme.border).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_TEXT_SECONDARY)
    };
    let prefix = if selected == 4 { "▶ " } else { "  " };
    lines.push(Line::from(Span::styled(format!("{}Back to Menu", prefix), back_style)));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("↑↓ Nav  •  ←→ Adjust  •  Enter/O Cycle  •  Q Back", Style::default().fg(COLOR_TEXT_SECONDARY))));
    
    let widget = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(widget, inner);
}
