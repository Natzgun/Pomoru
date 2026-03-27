use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub fn poll_event(timeout: Duration) -> Option<AppEvent> {
    if event::poll(timeout).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            // Only handle key press events (not release/repeat)
            if key.kind == KeyEventKind::Press {
                return Some(AppEvent::Key(key));
            }
        }
        return Some(AppEvent::Tick);
    }
    Some(AppEvent::Tick)
}
