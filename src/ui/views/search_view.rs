use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, Clear},
};
use crate::app::{App, LogEntry};

pub fn draw_search(f: &mut Frame, app: &App, search_text: &str, results: &[LogEntry], area: Rect) {
    let search_area = super::super::centered_rect(70, 80, area);
    f.render_widget(Clear, search_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Search input
            Constraint::Min(3),     // Results
        ])
        .split(search_area);

    // Search input
    let search_line = Line::from(vec![
        Span::raw("🔍 "),
        Span::styled(search_text, Style::default().fg(Color::Yellow)),
        Span::styled("_", Style::default().fg(Color::Gray)),
    ]);
    
    let search_input = Paragraph::new(search_line)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Search (Callsign, Mode, or Frequency)"));
    f.render_widget(search_input, chunks[0]);

    // Results
    let items: Vec<ListItem> = results.iter().map(|entry| {
        ListItem::new(Line::from(vec![
            Span::styled(
                entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                Style::default().fg(Color::Gray)
            ),
            Span::raw(" | "),
            Span::styled(&entry.callsign, Style::default().fg(Color::Yellow)),
            Span::raw(" | "),
            Span::styled(format!("{:.3}MHz", entry.frequency), Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled(&entry.mode, Style::default().fg(Color::Green)),
        ]))
    }).collect();

    let results_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!("Results ({})", results.len())));
    f.render_widget(results_list, chunks[1]);
}