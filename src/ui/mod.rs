use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear, List, ListItem},
};
use crate::app::{App, AppMode};

mod views;
mod layout;
pub use layout::centered_rect;

pub fn draw(f: &mut Frame, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Content
            Constraint::Length(1),  // Status
        ])
        .split(f.size());

    // Draw title
    let title = Paragraph::new("Amateur Radio Logbook")
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Draw content based on mode
    match app.mode {
        AppMode::NewEntry | AppMode::Edit => {
            draw_form(f, app, chunks[1]);
        },
        AppMode::Normal => {
            draw_log_list(f, app, chunks[1]);
        }
    }

    // Draw status message if any
    if let Some((message, is_error)) = &app.status_message {
        let style = if *is_error {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };
        
        let status = Paragraph::new(Line::from(vec![
            Span::styled(message, style)
        ]));
        f.render_widget(status, chunks[2]);
    }
}

fn draw_form(f: &mut Frame, app: &App, area: Rect) {
    // Create a centered form
    let form_area = centered_rect(60, 80, area);
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

    // Render form
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(form_title));
    f.render_widget(paragraph, form_area);

    // Draw cursor at current field position
    if let Some(field) = app.form.fields.get(app.form.current_field) {
        let x = 14 + field.cursor_position as u16; // 12 for label + 2 for ": "
        let y = app.form.current_field as u16 + 1; // +1 for border
        f.set_cursor(form_area.x + x, form_area.y + y);
    }
}

fn draw_log_list(f: &mut Frame, app: &App, area: Rect) {
    // Create list items from log entries
    let items: Vec<ListItem> = app.get_entries()
        .iter()
        .map(|entry| {
            ListItem::new(Line::from(vec![
                Span::raw(format!(
                    "{} - {} on {:.3}MHz {} RST: {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M"),
                    entry.callsign,
                    entry.frequency,
                    entry.mode,
                    entry.rst_sent.as_deref().unwrap_or("---")
                ))
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Log Entries (n: New, e: Edit, i: Import, x: Export)"));

    f.render_widget(list, area);
}