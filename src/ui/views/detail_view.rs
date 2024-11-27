use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use chrono::Timelike;
use crate::app::LogEntry;

pub fn draw_detail(f: &mut Frame, entry: &LogEntry, area: Rect) {
    let detail_area = super::super::centered_rect(70, 80, area);
    f.render_widget(Clear, detail_area);

    // Split the detail view into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Basic info
            Constraint::Length(3),  // Signal info
            Constraint::Length(3),  // Time info
            Constraint::Min(3),     // Notes
        ])
        .split(detail_area);

    // Basic QSO Information
    let basic_info = Line::from(vec![
        Span::raw("Callsign: "),
        Span::styled(&entry.callsign, Style::default().fg(Color::Yellow)),
        Span::raw(" | Frequency: "),
        Span::styled(format!("{:.3} MHz", entry.frequency), Style::default().fg(Color::Yellow)),
        Span::raw(" | Mode: "),
        Span::styled(&entry.mode, Style::default().fg(Color::Yellow)),
    ]);
    let basic = Paragraph::new(basic_info)
        .block(Block::default().borders(Borders::ALL).title("QSO Details"));
    f.render_widget(basic, chunks[0]);

    // Signal Information
    let signal_info = Line::from(vec![
        Span::raw("RST Sent: "),
        Span::styled(
            entry.rst_sent.as_deref().unwrap_or("---"),
            Style::default().fg(Color::Green)
        ),
        Span::raw(" | RST Received: "),
        Span::styled(
            entry.rst_received.as_deref().unwrap_or("---"),
            Style::default().fg(Color::Green)
        ),
    ]);
    let signal = Paragraph::new(signal_info)
        .block(Block::default().borders(Borders::ALL).title("Signal Report"));
    f.render_widget(signal, chunks[1]);

    // Time Information
    let time_info = Line::from(vec![
        Span::raw("Date: "),
        Span::styled(
            entry.timestamp.format("%Y-%m-%d").to_string(),
            Style::default().fg(Color::Cyan)
        ),
        Span::raw(" | Time: "),
        Span::styled(
            entry.timestamp.format("%H:%M:%S UTC").to_string(),
            Style::default().fg(Color::Cyan)
        ),
    ]);
    let time = Paragraph::new(time_info)
        .block(Block::default().borders(Borders::ALL).title("Time Information"));
    f.render_widget(time, chunks[2]);

    // Notes Section
    let notes = Paragraph::new(entry.notes.as_deref().unwrap_or("No notes"))
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(notes, chunks[3]);
}