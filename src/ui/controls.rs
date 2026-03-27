use ratatui::layout::Alignment;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::app::{App, InputMode, PomodoroState};
use crate::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Quit confirmation
    if app.confirm_quit {
        let confirm = Paragraph::new(Line::from(vec![
            Span::styled(
                " quit session? progress will be lost. ",
                Style::default().fg(theme::TIMER_PAUSED),
            ),
            hint_highlight("y", "yes"),
            Span::styled("  ", Style::default()),
            hint("any", "cancel"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(confirm, area);
        return;
    }

    let hints = match app.mode {
        InputMode::Setup => {
            let mut h = vec![
                hint("←→", "move"),
                sep(),
                hint("space", "toggle"),
                sep(),
                hint("H/L", "select range"),
                sep(),
                hint("c", "clear"),
                sep(),
                hint("↑↓", "duration"),
                sep(),
                hint("tab", "study/break"),
                sep(),
            ];
            if app.schedule.has_planned() {
                h.push(hint_highlight("enter", "start"));
                h.push(sep());
            }
            h.push(hint("q", "quit"));
            h
        }
        InputMode::Active => match app.state {
            PomodoroState::Studying | PomodoroState::Breaking => {
                vec![
                    hint("space", "pause"),
                    sep(),
                    hint("n", "skip"),
                    sep(),
                    hint("f", "finish"),
                    sep(),
                    hint("q", "quit"),
                ]
            }
            PomodoroState::Paused => {
                vec![
                    hint("space", "resume"),
                    sep(),
                    hint("f", "finish"),
                    sep(),
                    hint("q", "quit"),
                ]
            }
            PomodoroState::Done => {
                vec![
                    hint("r", "new session"),
                    sep(),
                    hint("q", "quit"),
                ]
            }
            _ => vec![hint("q", "quit")],
        },
    };

    let line = Paragraph::new(Line::from(hints)).alignment(Alignment::Center);
    frame.render_widget(line, area);
}

fn hint(key: &str, action: &str) -> Span<'static> {
    Span::styled(
        format!(" {} {}", key, action),
        Style::default().fg(theme::HINT_TEXT),
    )
}

fn hint_highlight(key: &str, action: &str) -> Span<'static> {
    Span::styled(
        format!(" {} {}", key, action),
        Style::default()
            .fg(theme::TIMER_STUDY)
            .add_modifier(Modifier::DIM),
    )
}

fn sep() -> Span<'static> {
    Span::styled("  ·", Style::default().fg(theme::BLOCK_UNPLANNED))
}
