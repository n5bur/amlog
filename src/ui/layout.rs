use ratatui::layout::{Layout, Direction, Constraint, Rect};

/// Helper function to create a centered rect using percentage of total area
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Create vertical layout
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Create horizontal layout for the middle section
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_rect() {
        let area = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        let centered = centered_rect(50, 50, area);
        
        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 50);
        assert_eq!(centered.x, 25);
        assert_eq!(centered.y, 25);
    }
}