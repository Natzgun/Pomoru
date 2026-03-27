use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::{App, InputMode};

pub struct Areas {
    pub timer: Rect,
    pub schedule: Rect,
    pub controls: Rect,
}

pub fn build_layout(area: Rect, app: &App) -> Areas {
    // Schedule height depends on mode and width
    let show_labels = area.width >= 48;
    let show_cursor = app.mode == InputMode::Setup;
    let show_config = app.mode == InputMode::Setup;

    // config(1?) + title(1) + blocks(1) + labels(1?) + cursor(1?) + legend(1)
    let schedule_height: u16 = 1 // title
        + 1 // blocks
        + 1 // legend
        + if show_labels { 1 } else { 0 }
        + if show_cursor { 1 } else { 0 }
        + if show_config { 1 } else { 0 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),                       // Timer (expands)
            Constraint::Length(schedule_height),       // Schedule bar
            Constraint::Length(2),                     // Controls
        ])
        .split(area);

    Areas {
        timer: chunks[0],
        schedule: chunks[1],
        controls: chunks[2],
    }
}
