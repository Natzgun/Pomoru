use chrono::Local;
use chrono::Timelike;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockState {
    Unplanned,
    Planned,
    Active { progress: f64 },
    Completed { order: usize },
}

pub struct Schedule {
    pub blocks: [BlockState; 24],
    pub current_hour: u8,
    completed_count: usize,
}

impl Schedule {
    pub fn new() -> Self {
        let hour = Local::now().hour() as u8;
        Self {
            blocks: [BlockState::Unplanned; 24],
            current_hour: hour,
            completed_count: 0,
        }
    }

    /// Toggle a block between Planned and Unplanned (only current or future)
    pub fn toggle_block(&mut self, hour: usize) {
        if hour >= 24 {
            return;
        }
        match self.blocks[hour] {
            BlockState::Unplanned => {
                self.blocks[hour] = BlockState::Planned;
            }
            BlockState::Planned => {
                self.blocks[hour] = BlockState::Unplanned;
            }
            _ => {} // Don't toggle active/completed/skipped
        }
    }

    /// Check if any blocks are planned or active
    pub fn has_planned(&self) -> bool {
        self.blocks.iter().any(|b| {
            matches!(b, BlockState::Planned | BlockState::Active { .. })
        })
    }

    /// Activate the next planned block. Returns the hour if found.
    pub fn activate_next(&mut self) -> Option<u8> {
        for i in 0..24 {
            if matches!(self.blocks[i], BlockState::Planned) {
                self.blocks[i] = BlockState::Active { progress: 0.0 };
                return Some(i as u8);
            }
        }
        None
    }

    /// Update progress on the currently active block
    pub fn update_active_progress(&mut self, progress: f64) {
        for block in self.blocks.iter_mut() {
            if matches!(block, BlockState::Active { .. }) {
                *block = BlockState::Active { progress };
                return;
            }
        }
    }

    /// Mark the current active block as completed
    pub fn complete_active(&mut self) {
        for block in self.blocks.iter_mut() {
            if matches!(block, BlockState::Active { .. }) {
                *block = BlockState::Completed {
                    order: self.completed_count,
                };
                self.completed_count += 1;
                return;
            }
        }
    }

    /// Number of completed blocks
    pub fn completed_total(&self) -> usize {
        self.completed_count
    }

    /// Refresh current hour from system clock
    pub fn refresh_hour(&mut self) {
        self.current_hour = Local::now().hour() as u8;
    }
}
