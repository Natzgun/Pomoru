use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::history::{History, SessionRecord};
use crate::notify;
use crate::schedule::Schedule;
use crate::timer::Timer;

#[derive(Clone, Copy, PartialEq)]
pub enum PomodoroState {
    Idle,
    Studying,
    Breaking,
    Paused,
    Done,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Setup,  // Picking hours + configuring durations
    Active, // Timer is running (study/break/paused)
}

/// Which duration field the user is editing
#[derive(Clone, Copy, PartialEq)]
pub enum DurationField {
    Study,
    Break,
}

pub struct App {
    pub state: PomodoroState,
    pub previous_state: PomodoroState,
    pub study_minutes: u64,
    pub break_minutes: u64,
    pub schedule: Schedule,
    pub timer: Timer,
    pub cursor: usize,
    pub mode: InputMode,
    pub editing_field: DurationField,
    pub should_quit: bool,
    pub completed_cycles: u32,
    pub tick_count: u64,
}

impl App {
    pub fn new(study_min: u64, break_min: u64) -> Self {
        let study_duration = Duration::from_secs(study_min * 60);
        Self {
            state: PomodoroState::Idle,
            previous_state: PomodoroState::Idle,
            study_minutes: study_min,
            break_minutes: break_min,
            schedule: Schedule::new(),
            timer: Timer::new(study_duration),
            cursor: chrono::Local::now().format("%H").to_string().parse().unwrap_or(8),
            mode: InputMode::Setup,
            editing_field: DurationField::Study,
            should_quit: false,
            completed_cycles: 0,
            tick_count: 0,
        }
    }

    pub fn study_duration(&self) -> Duration {
        Duration::from_secs(self.study_minutes * 60)
    }

    pub fn break_duration(&self) -> Duration {
        Duration::from_secs(self.break_minutes * 60)
    }

    pub fn handle_tick(&mut self) {
        self.tick_count += 1;
        self.schedule.refresh_hour();

        if self.mode == InputMode::Setup {
            // Update timer display to reflect current study duration
            if self.state == PomodoroState::Idle {
                self.timer.reset(self.study_duration());
            }
            return;
        }

        if self.timer.tick() {
            match self.state {
                PomodoroState::Studying => {
                    self.schedule.complete_active();
                    self.completed_cycles += 1;
                    notify::study_done(self.completed_cycles);
                    self.state = PomodoroState::Breaking;
                    self.timer.reset(self.break_duration());
                    self.timer.start();
                }
                PomodoroState::Breaking => {
                    if self.schedule.activate_next().is_some() {
                        notify::break_done();
                        self.state = PomodoroState::Studying;
                        self.timer.reset(self.study_duration());
                        self.timer.start();
                    } else {
                        notify::session_done(self.completed_cycles);
                        self.save_session();
                        self.state = PomodoroState::Done;
                    }
                }
                _ => {}
            }
        }

        if self.state == PomodoroState::Studying {
            self.schedule.update_active_progress(self.timer.progress());
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            InputMode::Setup => self.handle_setup_key(key),
            InputMode::Active => self.handle_active_key(key),
        }
    }

    fn handle_setup_key(&mut self, key: KeyEvent) {
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }

            // Navigate schedule (shift = move + toggle)
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                if shift || key.code == KeyCode::Char('H') {
                    self.schedule.toggle_block(self.cursor);
                }
                if self.cursor > 0 {
                    self.cursor -= 1;
                    if shift || key.code == KeyCode::Char('H') {
                        self.schedule.toggle_block(self.cursor);
                    }
                }
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                if shift || key.code == KeyCode::Char('L') {
                    self.schedule.toggle_block(self.cursor);
                }
                if self.cursor < 23 {
                    self.cursor += 1;
                    if shift || key.code == KeyCode::Char('L') {
                        self.schedule.toggle_block(self.cursor);
                    }
                }
            }

            // Toggle single hour block
            KeyCode::Char(' ') => {
                self.schedule.toggle_block(self.cursor);
            }

            // Clear all planned blocks
            KeyCode::Char('c') => {
                self.schedule.clear_all();
            }

            // Switch which duration field to edit
            KeyCode::Tab => {
                self.editing_field = match self.editing_field {
                    DurationField::Study => DurationField::Break,
                    DurationField::Break => DurationField::Study,
                };
            }

            // Adjust duration up
            KeyCode::Up | KeyCode::Char('+') | KeyCode::Char('k') => {
                match self.editing_field {
                    DurationField::Study => {
                        self.study_minutes = (self.study_minutes + 5).min(120);
                    }
                    DurationField::Break => {
                        self.break_minutes = (self.break_minutes + 5).min(60);
                    }
                }
            }

            // Adjust duration down
            KeyCode::Down | KeyCode::Char('-') | KeyCode::Char('j') => {
                match self.editing_field {
                    DurationField::Study => {
                        self.study_minutes = self.study_minutes.saturating_sub(5).max(5);
                    }
                    DurationField::Break => {
                        self.break_minutes = self.break_minutes.saturating_sub(5).max(5);
                    }
                }
            }

            // Start session
            KeyCode::Enter | KeyCode::Char('s') => {
                if self.schedule.has_planned() {
                    self.start_studying();
                }
            }

            _ => {}
        }
    }

    fn handle_active_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }

            // Pause / resume
            KeyCode::Char(' ') => self.toggle_pause(),

            // Finish session early
            KeyCode::Char('f') => {
                self.finish_session();
            }

            // Skip current phase (jump to next)
            KeyCode::Char('n') => {
                self.skip_phase();
            }

            // Reset everything
            KeyCode::Char('r') => {
                if self.state == PomodoroState::Done {
                    self.reset();
                }
            }

            _ => {}
        }
    }

    fn toggle_pause(&mut self) {
        match self.state {
            PomodoroState::Studying | PomodoroState::Breaking => {
                self.previous_state = self.state;
                self.state = PomodoroState::Paused;
                self.timer.pause();
            }
            PomodoroState::Paused => {
                self.state = self.previous_state;
                self.timer.resume();
            }
            _ => {}
        }
    }

    fn start_studying(&mut self) {
        if self.schedule.activate_next().is_some() {
            self.state = PomodoroState::Studying;
            self.timer.reset(self.study_duration());
            self.timer.start();
            self.mode = InputMode::Active;
        }
    }

    fn finish_session(&mut self) {
        if self.state == PomodoroState::Studying {
            self.schedule.complete_active();
            self.completed_cycles += 1;
        }
        notify::session_done(self.completed_cycles);
        self.save_session();
        self.state = PomodoroState::Done;
        self.timer.pause();
    }

    fn save_session(&self) {
        if self.completed_cycles == 0 {
            return;
        }
        let hours_planned: Vec<u8> = self
            .schedule
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| {
                matches!(
                    b,
                    crate::schedule::BlockState::Completed { .. }
                        | crate::schedule::BlockState::Active { .. }
                        | crate::schedule::BlockState::Planned
                )
            })
            .map(|(i, _)| i as u8)
            .collect();
        let record = SessionRecord {
            date: chrono::Local::now(),
            study_minutes: self.study_minutes,
            break_minutes: self.break_minutes,
            cycles_completed: self.completed_cycles,
            hours_planned,
        };
        let mut history = History::load();
        history.add_session(record);
    }

    fn skip_phase(&mut self) {
        match self.state {
            PomodoroState::Studying => {
                self.schedule.complete_active();
                self.completed_cycles += 1;
                self.state = PomodoroState::Breaking;
                self.timer.reset(self.break_duration());
                self.timer.start();
            }
            PomodoroState::Breaking => {
                if self.schedule.activate_next().is_some() {
                    self.state = PomodoroState::Studying;
                    self.timer.reset(self.study_duration());
                    self.timer.start();
                } else {
                    self.state = PomodoroState::Done;
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.schedule = Schedule::new();
        self.timer.reset(self.study_duration());
        self.state = PomodoroState::Idle;
        self.completed_cycles = 0;
        self.mode = InputMode::Setup;
    }

    pub fn state_label(&self) -> &str {
        match self.state {
            PomodoroState::Idle => "IDLE",
            PomodoroState::Studying => "STUDYING",
            PomodoroState::Breaking => "BREAK",
            PomodoroState::Paused => "PAUSED",
            PomodoroState::Done => "DONE",
        }
    }
}
