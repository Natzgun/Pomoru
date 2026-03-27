# pomoru

Minimal TUI Pomodoro timer with visual 24h schedule blocks.

## Build & Install

```bash
cargo build --release
cargo install --path .
```

## Run

```bash
pomoru                        # default: 50 min study, 10 min break
pomoru --study 25 --break 5   # custom durations
```

## Architecture

```
src/
├── main.rs          # CLI (clap), terminal lifecycle, main loop
├── app.rs           # State machine: Idle → Studying → Breaking → Done
├── timer.rs         # Countdown logic (tick-based, no threads)
├── schedule.rs      # 24h block model (Unplanned/Planned/Active/Completed)
├── event.rs         # Thin wrapper over crossterm event polling
├── theme.rs         # Color palette, gradients, pulse effects
└── ui/
    ├── mod.rs       # Draw entry point
    ├── layout.rs    # Vertical layout: timer | schedule bar | controls
    ├── timer_view.rs    # Centered MM:SS display + progress gauge
    ├── schedule_bar.rs  # 24h colored block bar with cursor
    └── controls.rs      # Context-sensitive key hints
```

## Key Conventions

- No async runtime — main loop uses `crossterm::event::poll` with 200ms timeout
- All rendering is stateless: UI functions take `&App` + `&mut Frame`, no mutation
- State transitions happen only in `app.rs` via `handle_tick()` and `handle_key()`
- Colors are centralized in `theme.rs`
- Two input modes: `Normal` (timer control) and `ScheduleEdit` (block selection)

## Dependencies

- `ratatui` 0.29 — TUI rendering
- `crossterm` 0.28 — terminal backend
- `clap` 4 — CLI argument parsing
- `chrono` 0.4 — wall-clock hour for schedule alignment
