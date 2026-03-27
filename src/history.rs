use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionRecord {
    pub date: DateTime<Local>,
    pub study_minutes: u64,
    pub break_minutes: u64,
    pub cycles_completed: u32,
    pub hours_planned: Vec<u8>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct History {
    pub sessions: Vec<SessionRecord>,
}

fn history_path() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pomoru");
    let _ = fs::create_dir_all(&data_dir);
    data_dir.join("history.json")
}

impl History {
    pub fn load() -> Self {
        let path = history_path();
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = history_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn add_session(&mut self, record: SessionRecord) {
        self.sessions.push(record);
        self.save();
    }

    /// Sessions for a given date
    pub fn sessions_on(&self, date: NaiveDate) -> Vec<&SessionRecord> {
        self.sessions
            .iter()
            .filter(|s| s.date.date_naive() == date)
            .collect()
    }

    /// Total cycles completed on a date
    pub fn cycles_on(&self, date: NaiveDate) -> u32 {
        self.sessions_on(date).iter().map(|s| s.cycles_completed).sum()
    }

    /// Total study minutes on a date
    pub fn study_minutes_on(&self, date: NaiveDate) -> u64 {
        self.sessions_on(date)
            .iter()
            .map(|s| s.study_minutes * s.cycles_completed as u64)
            .sum()
    }

    /// Streak: consecutive days with at least one session, ending today or yesterday
    pub fn streak(&self) -> u32 {
        let today = Local::now().date_naive();
        let mut streak = 0u32;
        let mut day = today;

        loop {
            if self.cycles_on(day) > 0 {
                streak += 1;
                day -= chrono::Duration::days(1);
            } else if day == today {
                // Today has no sessions yet, check from yesterday
                day -= chrono::Duration::days(1);
            } else {
                break;
            }
        }
        streak
    }

    /// Days with sessions in the last N days
    pub fn active_days(&self, last_n_days: u32) -> u32 {
        let today = Local::now().date_naive();
        (0..last_n_days)
            .filter(|&i| {
                let day = today - chrono::Duration::days(i as i64);
                self.cycles_on(day) > 0
            })
            .count() as u32
    }

    /// Total study minutes in the last N days
    pub fn total_minutes_last(&self, last_n_days: u32) -> u64 {
        let today = Local::now().date_naive();
        (0..last_n_days)
            .map(|i| {
                let day = today - chrono::Duration::days(i as i64);
                self.study_minutes_on(day)
            })
            .sum()
    }
}
