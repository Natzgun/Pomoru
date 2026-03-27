use std::process::Command;

/// Send a desktop notification via notify-send (Linux)
pub fn send(title: &str, body: &str) {
    let _ = Command::new("notify-send")
        .arg("--app-name=pomoru")
        .arg("--urgency=normal")
        .arg(title)
        .arg(body)
        .spawn();
}

/// Terminal bell (works everywhere)
pub fn bell() {
    print!("\x07");
}

pub fn study_done(cycles: u32) {
    bell();
    send(
        "Break time!",
        &format!("Study session #{} complete. Take a break.", cycles),
    );
}

pub fn break_done() {
    bell();
    send("Back to work!", "Break is over. Time to study.");
}

pub fn session_done(cycles: u32) {
    bell();
    send(
        "Session complete!",
        &format!("All done! {} cycles completed.", cycles),
    );
}
