mod views;
mod layout;

use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::{App, AppMode};

use self::views::{draw_form, draw_log_list};
pub use layout::centered_rect;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(1),     // Content
            Constraint::Length(1),  // Status bar
        ])
        .split(f.size());

    // Draw title
    let version = env!("CARGO_PKG_VERSION");
    let title = Paragraph::new(format!("amlog v{}", version))
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

    if !app.deleted_entries.is_empty() {
        let status = app.status_message.as_ref().map(|(msg, is_error)| {
            if *is_error {
                (msg.clone(), true)
            } else {
                (format!("{} ({} deletions in buffer)", msg, app.deleted_entries.len()), false)
            }
        });
        
        if let Some((message, is_error)) = status {
            let style = if is_error {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };
            let status = Paragraph::new(Line::from(vec![
                Span::styled(&message, style)
            ]));
            f.render_widget(status, chunks[2]);
        }
    }
}
