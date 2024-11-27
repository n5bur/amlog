use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, BarChart},
};
use std::collections::{HashMap, HashSet};
use crate::app::LogEntry;

pub fn draw_stats(f: &mut Frame, entries: &[LogEntry], area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Summary
            Constraint::Min(10),    // Mode chart
            Constraint::Length(10), // Band chart
        ])
        .split(area);

    // Summary statistics
    draw_summary_stats(f, entries, chunks[0]);
    
    // Mode distribution
    draw_mode_chart(f, entries, chunks[1]);
    
    // Band distribution
    draw_band_chart(f, entries, chunks[2]);
}

fn draw_summary_stats(f: &mut Frame, entries: &[LogEntry], area: Rect) {
    let total_qsos = entries.len();
    let unique_calls: HashSet<_> = entries.iter().map(|e| &e.callsign).collect();
    let unique_modes: HashSet<_> = entries.iter().map(|e| &e.mode).collect();

    let summary = Line::from(vec![
        Span::raw("Total QSOs: "),
        Span::styled(total_qsos.to_string(), Style::default().fg(Color::Yellow)),
        Span::raw(" | Unique Calls: "),
        Span::styled(unique_calls.len().to_string(), Style::default().fg(Color::Green)),
        Span::raw(" | Modes Used: "),
        Span::styled(unique_modes.len().to_string(), Style::default().fg(Color::Cyan)),
    ]);

    let paragraph = Paragraph::new(summary)
        .block(Block::default().borders(Borders::ALL).title("Log Statistics"));
    f.render_widget(paragraph, area);
}

fn draw_mode_chart(f: &mut Frame, entries: &[LogEntry], area: Rect) {
    let mut mode_counts: HashMap<String, u64> = HashMap::new();
    for entry in entries {
        *mode_counts.entry(entry.mode.clone()).or_default() += 1;
    }

    let mut mode_data: Vec<(&str, u64)> = mode_counts
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();
    mode_data.sort_by_key(|k| std::cmp::Reverse(k.1));

    let mode_chart = BarChart::default()
        .block(Block::default().title("QSOs by Mode").borders(Borders::ALL))
        .bar_width(8)
        .group_gap(2)
        .bar_gap(1)
        .value_style(Style::default().fg(Color::Yellow))
        .data(mode_data.as_slice());

    f.render_widget(mode_chart, area);
}

fn draw_band_chart(f: &mut Frame, entries: &[LogEntry], area: Rect) {
    let mut band_counts: HashMap<String, u64> = HashMap::new();
    
    for entry in entries {
        let band = match entry.frequency {
            f if f >= 1.8 && f <= 2.0 => "160m",
            f if f >= 3.5 && f <= 4.0 => "80m",
            f if f >= 7.0 && f <= 7.3 => "40m",
            f if f >= 14.0 && f <= 14.35 => "20m",
            f if f >= 21.0 && f <= 21.45 => "15m",
            f if f >= 28.0 && f <= 29.7 => "10m",
            f if f >= 50.0 && f <= 54.0 => "6m",
            f if f >= 144.0 && f <= 148.0 => "2m",
            _ => "Other",
        };
        *band_counts.entry(band.to_string()).or_default() += 1;
    }

    let mut band_data: Vec<(&str, u64)> = band_counts
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();
    band_data.sort_by_key(|k| std::cmp::Reverse(k.1));

    let band_chart = BarChart::default()
        .block(Block::default().title("QSOs by Band").borders(Borders::ALL))
        .bar_width(8)
        .bar_gap(1)
        .group_gap(2)
        .value_style(Style::default().fg(Color::Cyan))
        .data(band_data.as_slice());

    f.render_widget(band_chart, area);
}