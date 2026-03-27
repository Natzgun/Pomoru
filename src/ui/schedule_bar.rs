use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, DurationField, InputMode};
use crate::schedule::BlockState;
use crate::theme;

/// Layout mode based on available terminal width
enum BarMode {
    Wide,    // 3-char blocks + "HHa" labels (needs ~96 cols)
    Medium,  // 2-char blocks + "HH" labels  (needs ~72 cols)
    Compact, // 1-char blocks + sparse labels (needs ~48 cols)
    Tiny,    // 1-char blocks, no labels      (needs ~26 cols)
}

fn bar_mode(width: u16) -> BarMode {
    if width >= 96 {
        BarMode::Wide
    } else if width >= 72 {
        BarMode::Medium
    } else if width >= 48 {
        BarMode::Compact
    } else {
        BarMode::Tiny
    }
}

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mode = bar_mode(area.width);
    let show_labels = !matches!(mode, BarMode::Tiny);
    let show_config = app.mode == InputMode::Setup;

    // Rows: config? + title + blocks + labels? + cursor? + legend
    let mut constraints = Vec::new();
    if show_config {
        constraints.push(Constraint::Length(1)); // duration config
    }
    constraints.push(Constraint::Length(1)); // title
    constraints.push(Constraint::Length(1)); // blocks
    if show_labels {
        constraints.push(Constraint::Length(1)); // labels
    }
    if app.mode == InputMode::Setup {
        constraints.push(Constraint::Length(1)); // cursor
    }
    constraints.push(Constraint::Length(1)); // legend

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut row_idx = 0;

    // Duration config (only in setup)
    if show_config {
        let config_line = build_duration_config(app);
        let config = Paragraph::new(Line::from(config_line)).alignment(Alignment::Center);
        frame.render_widget(config, rows[row_idx]);
        row_idx += 1;
    }

    // Title
    let title_text = if app.mode == InputMode::Setup {
        "select your study hours"
    } else {
        "schedule"
    };
    let title = Paragraph::new(Line::from(Span::styled(
        title_text,
        Style::default().fg(theme::DIM_TEXT),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(title, rows[row_idx]);
    row_idx += 1;

    // Build spans
    let total_completed = app.schedule.completed_total();
    let mut block_spans = Vec::new();
    let mut label_spans = Vec::new();
    let mut cursor_spans = Vec::new();

    for i in 0..24 {
        let block = &app.schedule.blocks[i];
        let is_current = i as u8 == app.schedule.current_hour;

        let (block_ch, block_style) =
            render_block(block, is_current, total_completed, app.tick_count, &mode);
        block_spans.push(Span::styled(block_ch, block_style));

        // Hour labels
        match mode {
            BarMode::Wide => {
                let lbl = format_hour_ampm(i);
                label_spans.push(Span::styled(lbl, hour_style(is_current)));
            }
            BarMode::Medium => {
                let lbl = format!("{:02} ", i);
                label_spans.push(Span::styled(lbl, hour_style(is_current)));
            }
            BarMode::Compact => {
                if i % 3 == 0 {
                    label_spans.push(Span::styled(
                        format!("{:02}", i),
                        hour_style(is_current),
                    ));
                } else {
                    label_spans.push(Span::styled(
                        "  ",
                        Style::default().fg(theme::BLOCK_UNPLANNED),
                    ));
                }
            }
            BarMode::Tiny => {}
        }

        // Cursor
        let cursor_w = match mode {
            BarMode::Wide => " ▲  ",
            BarMode::Medium => "▲  ",
            BarMode::Compact => "▲ ",
            BarMode::Tiny => "▲",
        };
        let cursor_blank = match mode {
            BarMode::Wide => "    ",
            BarMode::Medium => "   ",
            BarMode::Compact => "  ",
            BarMode::Tiny => " ",
        };
        if app.mode == InputMode::Setup && i == app.cursor {
            cursor_spans.push(Span::styled(
                cursor_w,
                Style::default().fg(theme::CURSOR_COLOR),
            ));
        } else {
            cursor_spans.push(Span::raw(cursor_blank));
        }
    }

    // Blocks row
    let blocks_line = Paragraph::new(Line::from(block_spans)).alignment(Alignment::Center);
    frame.render_widget(blocks_line, rows[row_idx]);
    row_idx += 1;

    // Labels
    if show_labels {
        let labels_line = Paragraph::new(Line::from(label_spans)).alignment(Alignment::Center);
        frame.render_widget(labels_line, rows[row_idx]);
        row_idx += 1;
    }

    // Cursor
    if app.mode == InputMode::Setup {
        let cursor_line = Paragraph::new(Line::from(cursor_spans)).alignment(Alignment::Center);
        frame.render_widget(cursor_line, rows[row_idx]);
        row_idx += 1;
    }

    // Legend
    let legend = build_legend(app);
    let legend_line = Paragraph::new(Line::from(legend)).alignment(Alignment::Center);
    frame.render_widget(legend_line, rows[row_idx]);
}

fn build_duration_config(app: &App) -> Vec<Span<'static>> {
    let dim = Style::default().fg(theme::HINT_TEXT);
    let active = Style::default()
        .fg(theme::CURSOR_COLOR)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(theme::DIM_TEXT);

    let study_style = if app.editing_field == DurationField::Study {
        active
    } else {
        inactive
    };
    let break_style = if app.editing_field == DurationField::Break {
        active
    } else {
        inactive
    };

    let arrow = if app.editing_field == DurationField::Study {
        " ↕"
    } else {
        ""
    };
    let arrow_b = if app.editing_field == DurationField::Break {
        " ↕"
    } else {
        ""
    };

    vec![
        Span::styled("study ", dim),
        Span::styled(format!("{}min", app.study_minutes), study_style),
        Span::styled(arrow, Style::default().fg(theme::CURSOR_COLOR)),
        Span::styled("    ", dim),
        Span::styled("break ", dim),
        Span::styled(format!("{}min", app.break_minutes), break_style),
        Span::styled(arrow_b, Style::default().fg(theme::CURSOR_COLOR)),
    ]
}

fn render_block(
    block: &BlockState,
    is_current: bool,
    total_completed: usize,
    tick: u64,
    mode: &BarMode,
) -> (String, Style) {
    let (fill, empty, planned_ch) = match mode {
        BarMode::Wide => ("███ ", "░░░ ", "▓▓▓ "),
        BarMode::Medium => ("██ ", "░░ ", "▓▓ "),
        BarMode::Compact => ("█ ", "░ ", "▓ "),
        BarMode::Tiny => ("█", "░", "▓"),
    };

    match block {
        BlockState::Unplanned => {
            let color = if is_current {
                theme::BLOCK_UNPLANNED_CURRENT
            } else {
                theme::BLOCK_UNPLANNED
            };
            (empty.to_string(), Style::default().fg(color))
        }
        BlockState::Planned => (
            planned_ch.to_string(),
            Style::default().fg(theme::BLOCK_PLANNED),
        ),
        BlockState::Active { .. } => (fill.to_string(), theme::pulse_style(tick)),
        BlockState::Completed { order } => {
            let color = theme::completed_gradient(*order, total_completed);
            (fill.to_string(), Style::default().fg(color))
        }
    }
}

fn format_hour_ampm(h: usize) -> String {
    let (display, suffix) = match h {
        0 => (12, "a"),
        1..=11 => (h, "a"),
        12 => (12, "p"),
        _ => (h - 12, "p"),
    };
    format!("{:>2}{} ", display, suffix)
}

fn hour_style(is_current: bool) -> Style {
    if is_current {
        Style::default()
            .fg(theme::TIMER_STUDY)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::DIM_TEXT)
    }
}

fn build_legend(app: &App) -> Vec<Span<'static>> {
    let dim = Style::default().fg(theme::HINT_TEXT);
    let mut spans = vec![];

    spans.push(Span::styled("░", Style::default().fg(theme::BLOCK_UNPLANNED)));
    spans.push(Span::styled(" free  ", dim));

    spans.push(Span::styled("▓", Style::default().fg(theme::BLOCK_PLANNED)));
    spans.push(Span::styled(" planned  ", dim));

    if app.completed_cycles > 0 {
        spans.push(Span::styled(
            "█",
            Style::default().fg(theme::completed_gradient(0, 1)),
        ));
        spans.push(Span::styled(" done", dim));
    }

    spans
}
