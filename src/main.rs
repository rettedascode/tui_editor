mod app;
mod editor;
mod file_explorer;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new()?;
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            ui::ui(f, &mut app);
            if app.show_help {
                ui::render_help(f, &app);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if app.show_help {
                app.show_help = false;
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('n') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.new_file();
                    }
                }
                KeyCode::Char('o') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.open_file_dialog();
                    }
                }
                KeyCode::Char('s') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        if let Err(e) = app.save_current_file() {
                            app.set_status_message(format!("Error saving file: {}", e));
                        }
                    }
                }
                KeyCode::Tab => {
                    app.toggle_panel();
                }
                KeyCode::F(1) => {
                    app.show_help = !app.show_help;
                }
                _ => {
                    app.handle_input(key);
                }
            }
        }
    }
}
