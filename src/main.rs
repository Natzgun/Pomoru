mod app;
mod ascii;
mod event;
mod history;
mod notify;
mod schedule;
mod theme;
mod timer;
mod ui;

use std::io;
use std::time::Duration;

use clap::{Parser, Subcommand};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;
use event::{poll_event, AppEvent};
use history::History;

#[derive(Parser)]
#[command(name = "pomoru", about = "A minimal TUI Pomodoro timer")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Study duration in minutes
    #[arg(short, long, default_value_t = 50)]
    study: u64,

    /// Break duration in minutes
    #[arg(short, long, default_value_t = 10)]
    r#break: u64,

    /// Preset: deep (50/10), classic (25/5), short (15/3)
    #[arg(short, long)]
    preset: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show study statistics
    Stats,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Stats) = cli.command {
        print_stats();
        return Ok(());
    }

    // Resolve durations (preset overrides flags)
    let (study, brk) = match cli.preset.as_deref() {
        Some("deep") => (50, 10),
        Some("classic") => (25, 5),
        Some("short") => (15, 3),
        Some(other) => {
            eprintln!("Unknown preset '{}'. Available: deep, classic, short", other);
            return Ok(());
        }
        None => (cli.study, cli.r#break),
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run app
    let mut app = App::new(study, brk);
    let result = run_loop(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(200);

    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        match poll_event(tick_rate) {
            Some(AppEvent::Key(key)) => app.handle_key(key),
            Some(AppEvent::Tick) => app.handle_tick(),
            None => {}
        }

        app.handle_tick();

        if app.should_quit {
            return Ok(());
        }
    }
}

fn print_stats() {
    let history = History::load();
    let today = chrono::Local::now().date_naive();

    let today_minutes = history.study_minutes_on(today);
    let today_cycles = history.cycles_on(today);
    let week_minutes = history.total_minutes_last(7);
    let week_days = history.active_days(7);
    let streak = history.streak();
    let total_sessions = history.sessions.len();

    println!("  pomoru stats");
    println!("  ────────────");
    println!();
    println!("  today     {}h {}m  ({} cycles)",
        today_minutes / 60, today_minutes % 60, today_cycles);
    println!("  week      {}h {}m  ({}/7 days)",
        week_minutes / 60, week_minutes % 60, week_days);
    println!("  streak    {} day{}", streak, if streak == 1 { "" } else { "s" });
    println!("  sessions  {} total", total_sessions);
    println!();

    // Last 7 days bar
    println!("  last 7 days:");
    for i in (0..7).rev() {
        let day = today - chrono::Duration::days(i);
        let mins = history.study_minutes_on(day);
        let bar_len = (mins / 10) as usize; // 10 min per block
        let bar: String = "█".repeat(bar_len.min(30));
        let label = day.format("%a");
        if mins > 0 {
            println!("  {} {:>3}m {}", label, mins, bar);
        } else {
            println!("  {} {:>3}m", label, mins);
        }
    }
    println!();
}
