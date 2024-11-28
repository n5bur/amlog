use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
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

    let mut list_state = ListState::default();
    list_state.select(app.selected_index());

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Log Entries (↑/↓: Select, e: Edit, n: New, i: Import, x: Export)"))
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)  // Changed to background color
                .fg(Color::Black)   // Black text on yellow background
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("➤ ");    // Changed to a clearer symbol

    // Use the stateful rendering
    f.render_stateful_widget(list, area, &mut list_state);
}