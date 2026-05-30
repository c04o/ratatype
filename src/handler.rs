use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::time::Instant;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    // skip key release events; avoid double-triggering
    if key_event.kind != KeyEventKind::Press {
        return;
    }

    match key_event.code {
        // quit on esc
        KeyCode::Esc => app.should_quit = true,

        // remove last char on backspace
        KeyCode::Backspace => {
            app.user_input.pop();
        }
        // handle typing
        KeyCode::Char(c) => {
            // start timer on first key
            if app.start_time.is_none() {
                app.start_time = Some(Instant::now());
            }
            app.user_input.push(c);
        }
        _ => {}
    }
}
