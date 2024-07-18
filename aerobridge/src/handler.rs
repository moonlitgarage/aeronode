use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<bool> {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
            return Ok(true); // Signal that we want to quit
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
                return Ok(true); // Signal that we want to quit
            }
        }
        // Counter handlers
        KeyCode::Right => {
            app.increment_counter();
        }
        KeyCode::Left => {
            app.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(false) // Signal that we don't want to quit
}