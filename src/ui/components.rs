// Reusable UI components

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
    Frame,
};

/// Draw a popup window with title and content
pub fn draw_popup(f: &mut Frame, size: Rect, title: &str, lines: Vec<&str>, border_color: Color) {
    let panel_width = 40;
    let panel_height = lines.len() as u16 + 4;
    let panel_area = Rect {
        x: (size.width.saturating_sub(panel_width)) / 2,
        y: (size.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    f.render_widget(Clear, panel_area);
    f.render_widget(block.clone(), panel_area);

    let inner = block.inner(panel_area);
    let text_lines: Vec<Line> = lines.iter().map(|l| Line::from(*l)).collect();
    let paragraph = Paragraph::new(text_lines);
    f.render_widget(paragraph, inner);
}
