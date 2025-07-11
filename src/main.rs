mod app;
mod editor;
mod file_explorer;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
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

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// File to open
    #[arg(short = 'f', long = "file")]
    file: Option<String>,

    /// Directory to open
    #[arg(short = 'd', long = "dir")]
    dir: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new()?;

    // If a directory is specified, set it as the root for the file explorer
    if let Some(dir) = cli.dir {
        app.set_directory(dir)?;
    }

    // If a file is specified, open it in a new tab
    if let Some(file) = cli.file {
        app.open_file(file)?;
    }

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
