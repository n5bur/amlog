use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

// Import the required types from our lib
use amlog::{App, AppMode, ui};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new().expect("Failed to create app");

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('n') => app.enter_new_mode(),
                            KeyCode::Char('e') => app.edit_selected_entry(),
                            KeyCode::Up => app.select_previous(),
                            KeyCode::Down => app.select_next(),
                            KeyCode::Char('j') => app.select_next(),
                            KeyCode::Char('k') => app.select_previous(),
                            KeyCode::Char('i') => {
                                app.set_status("Import feature coming soon");
                            },
                            KeyCode::Char('x') => {
                                app.set_status("Export feature coming soon");
                            },
                            KeyCode::Char('d') => {
                                app.delete_current_entry();
                            },
                            KeyCode::Char('u') => {
                                app.undo_delete();
                            },
                            _ => {}
                        }
                    },
                    AppMode::NewEntry | AppMode::Edit => {
                        match key.code {
                            KeyCode::Esc => app.enter_normal_mode(),
                            KeyCode::Tab => {
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    app.previous_field();
                                } else {
                                    app.next_field();
                                }
                            },
                            KeyCode::Enter => {
                                if app.form.is_valid() {
                                    app.save_entry();
                                    app.enter_normal_mode();
                                } else {
                                    app.set_error("Please fill in all required fields");
                                }
                            },
                            KeyCode::Char(c) => app.handle_input(c),
                            KeyCode::Backspace => app.handle_backspace(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}