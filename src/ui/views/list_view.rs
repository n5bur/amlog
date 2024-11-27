use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use crate::app::App;

pub fn draw_log_list(f: &mut Frame, app: &App, area: Rect) {
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
            .title("Log Entries (n: New, e: Edit, i: Import, x: Export)"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}