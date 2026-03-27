use ratatui::style::{Color, Modifier, Style};

// Timer state colors - subtle, not overwhelming
pub const TIMER_IDLE: Color = Color::Rgb(140, 140, 150);
pub const TIMER_STUDY: Color = Color::Rgb(120, 200, 140);
pub const TIMER_BREAK: Color = Color::Rgb(100, 180, 220);
pub const TIMER_PAUSED: Color = Color::Rgb(200, 190, 80);

// Schedule block colors
pub const BLOCK_UNPLANNED: Color = Color::Rgb(45, 45, 50);
pub const BLOCK_UNPLANNED_CURRENT: Color = Color::Rgb(65, 65, 75);
pub const BLOCK_PLANNED: Color = Color::Rgb(50, 80, 60);

pub const DIM_TEXT: Color = Color::Rgb(100, 100, 110);
pub const HINT_TEXT: Color = Color::Rgb(80, 80, 90);
pub const CURSOR_COLOR: Color = Color::Rgb(220, 200, 100);

/// Gradient for completed blocks: earlier = dimmer, later = brighter
pub fn completed_gradient(index: usize, total: usize) -> Color {
    let t = if total <= 1 {
        1.0
    } else {
        (index as f64 + 1.0) / total as f64
    };
    let g = (80.0 + 120.0 * t) as u8;
    let r = (20.0 + 30.0 * t) as u8;
    Color::Rgb(r, g, 50)
}

/// Pulse effect for active block based on tick counter
pub fn pulse_style(tick: u64) -> Style {
    let phase = (tick as f64 * 0.12).sin() * 0.5 + 0.5;
    let g = (140.0 + 80.0 * phase) as u8;
    let r = (40.0 + 30.0 * phase) as u8;
    Style::default().fg(Color::Rgb(r, g, 90))
}

/// Style for the label below timer
pub fn state_label_style(studying: bool, breaking: bool, paused: bool) -> Style {
    let color = if paused {
        TIMER_PAUSED
    } else if studying {
        TIMER_STUDY
    } else if breaking {
        TIMER_BREAK
    } else {
        TIMER_IDLE
    };
    Style::default().fg(color).add_modifier(Modifier::DIM)
}
