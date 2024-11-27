use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
};
use crate::app::App;

pub fn draw_help(f: &mut Frame, _app: &App, area: Rect) {
    let help_area = super::super::centered_rect(60, 70, area);
    f.render_widget(Clear, help_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(2),     // Content
        ])
        .split(help_area);

    // Title
    let title = Paragraph::new("Ham Radio Logbook Help")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Help content
    let help_text = vec![
        Line::from(vec![
            Span::styled("Global Commands", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("q      - "),
            Span::styled("Quit application", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("?      - "),
            Span::styled("Toggle help view", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Log Entry Commands", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("n      - "),
            Span::styled("New log entry", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("e      - "),
            Span::styled("Edit selected entry", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("d      - "),
            Span::styled("View entry details", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Form Navigation", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Tab    - "),
            Span::styled("Next field", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("S-Tab  - "),
            Span::styled("Previous field", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Enter  - "),
            Span::styled("Save entry", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("Esc    - "),
            Span::styled("Cancel/Return", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Import/Export", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("i      - "),
            Span::styled("Import ADIF file", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("x      - "),
            Span::styled("Export to ADIF", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let help_content = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(help_content, chunks[1]);
}