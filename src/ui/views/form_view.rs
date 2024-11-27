use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::{App, AppMode};

pub fn draw_form(f: &mut Frame, app: &App, area: Rect) {
    let form_area = super::super::centered_rect(60, 80, area);
    f.render_widget(Clear, form_area);

    let form_title = match app.mode {
        AppMode::NewEntry => "New Log Entry (Tab to navigate, Enter to save)",
        AppMode::Edit => "Edit Log Entry (Tab to navigate, Enter to save)",
        _ => "Log Entry Form",
    };

    // Create form lines with current field highlighted
    let lines: Vec<Line> = app.form.fields.iter().enumerate()
        .map(|(i, field)| {
            let style = if i == app.form.current_field {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            Line::from(vec![
                Span::styled(format!("{:<12}", field.label), style),
                Span::raw(": "),
                Span::styled(&field.value, style),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(form_title));
    f.render_widget(paragraph, form_area);

    // Draw cursor at current field position
    if let Some(field) = app.form.fields.get(app.form.current_field) {
        let x = 14 + field.cursor_position as u16;
        let y = app.form.current_field as u16 + 1;
        f.set_cursor(form_area.x + x, form_area.y + y);
    }
}