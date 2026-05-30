pub mod app;
pub mod handler;
pub mod ui;

use crate::app::App;
use crossterm::event::{self, Event};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> io::Result<()> {
    // init terminal
    // take keyboard control
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    // use alt screen
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // connect ratatui
    let backend = CrosstermBackend::new(stdout);
    // start terminal
    let mut terminal = Terminal::new(backend)?;

    // create state
    let mut app = App::new();

    // main loop
    while !app.should_quit {
        // draw screen
        terminal.draw(|f| ui::render(&app, f))?;

        // wait for input
        // 60fps-ish
        if let Ok(true) = event::poll(std::time::Duration::from_millis(16)) {
            if let Event::Key(key) = event::read()? {
                // process key
                handler::handle_key_events(key, &mut app);
            }
        }
    }

    // clean up
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}
