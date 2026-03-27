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

use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;
use event::{poll_event, AppEvent};

#[derive(Parser)]
#[command(name = "pomoru", about = "A minimal TUI Pomodoro timer")]
struct Cli {
    /// Study duration in minutes
    #[arg(short, long, default_value_t = 50)]
    study: u64,

    /// Break duration in minutes
    #[arg(short, long, default_value_t = 10)]
    r#break: u64,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run app
    let mut app = App::new(cli.study, cli.r#break);
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

        // Also tick on key events so timer stays accurate
        app.handle_tick();

        if app.should_quit {
            return Ok(());
        }
    }
}
