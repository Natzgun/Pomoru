use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, Paragraph};
use ratatui::Frame;

use crate::app::{App, PomodoroState};
use crate::ascii;
use crate::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let timer_color = match app.state {
        PomodoroState::Idle => theme::TIMER_IDLE,
        PomodoroState::Studying => theme::TIMER_STUDY,
        PomodoroState::Breaking => theme::TIMER_BREAK,
        PomodoroState::Paused => theme::TIMER_PAUSED,
        PomodoroState::Done => theme::TIMER_STUDY,
    };

    let wide = area.width >= 50;
    let big_digits = area.width >= 34;
    let digit_height: u16 = if big_digits { 5 } else { 3 };

    // art(5) + spacer(1) + digits(3or5) + label(1) + gauge(1) + cycles(1) = 12 or 14
    let content_height = 5 + 1 + digit_height + 1 + 1 + 1;

    // Vertical centering
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(content_height),
            Constraint::Min(0),
        ])
        .split(area);

    let content_area = v_chunks[1];

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),            // state ASCII art
            Constraint::Length(1),            // spacer
            Constraint::Length(digit_height), // timer digits
            Constraint::Length(1),            // state label
            Constraint::Length(1),            // progress bar
            Constraint::Length(1),            // cycles
        ])
        .split(content_area);

    // State ASCII art
    let art_key = match app.state {
        PomodoroState::Idle => "idle",
        PomodoroState::Studying => "studying",
        PomodoroState::Breaking => "break",
        PomodoroState::Paused => "paused",
        PomodoroState::Done => "done",
    };
    let art = ascii::state_art(art_key);
    let art_style = Style::default()
        .fg(timer_color)
        .add_modifier(Modifier::DIM);
    let art_lines: Vec<Line> = art
        .iter()
        .map(|l| Line::from(Span::styled(*l, art_style)))
        .collect();
    let art_widget = Paragraph::new(art_lines).alignment(Alignment::Center);
    frame.render_widget(art_widget, rows[0]);

    // Timer digits (rounded style)
    let time_str = app.timer.display();
    let big_lines = ascii::render_big_time(&time_str, area.width);
    let timer_style = Style::default()
        .fg(timer_color)
        .add_modifier(Modifier::BOLD);
    let timer_lines: Vec<Line> = big_lines
        .iter()
        .map(|l| Line::from(Span::styled(l.as_str(), timer_style)))
        .collect();
    let timer_widget = Paragraph::new(timer_lines).alignment(Alignment::Center);
    frame.render_widget(timer_widget, rows[2]);

    // State label
    let label = Paragraph::new(Line::from(Span::styled(
        app.state_label(),
        theme::state_label_style(
            app.state == PomodoroState::Studying,
            app.state == PomodoroState::Breaking,
            app.state == PomodoroState::Paused,
        ),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(label, rows[3]);

    // Progress bar
    if matches!(
        app.state,
        PomodoroState::Studying | PomodoroState::Breaking | PomodoroState::Paused
    ) {
        let pct = if wide { 30 } else { 50 };
        let gauge_area = centered_rect(pct, rows[4]);
        let progress = app.timer.progress();
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(timer_color).bg(theme::BLOCK_UNPLANNED))
            .ratio(progress.clamp(0.0, 1.0))
            .label("");
        frame.render_widget(gauge, gauge_area);
    }

    // Cycles completed
    if app.completed_cycles > 0 || app.state == PomodoroState::Done {
        let cycles_text = if app.state == PomodoroState::Done {
            format!("{} cycles completed", app.completed_cycles)
        } else {
            format!("{} done", app.completed_cycles)
        };
        let cycles = Paragraph::new(Line::from(Span::styled(
            cycles_text,
            Style::default().fg(theme::DIM_TEXT),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(cycles, rows[5]);
    }
}

fn centered_rect(percent_x: u16, area: Rect) -> Rect {
    let side = (100u16.saturating_sub(percent_x)) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(side),
            Constraint::Percentage(percent_x),
            Constraint::Percentage(side),
        ])
        .split(area)[1]
}
