/// Big digit font - 5 lines tall, 5 chars wide
/// Uses rounded box-drawing characters for a softer look
pub fn big_digit(ch: char) -> [&'static str; 5] {
    match ch {
        '0' => [
            " ╭─╮ ",
            " │ │ ",
            " │ │ ",
            " │ │ ",
            " ╰─╯ ",
        ],
        '1' => [
            "  ╷  ",
            " ╶│  ",
            "  │  ",
            "  │  ",
            " ─┴─ ",
        ],
        '2' => [
            " ╭─╮ ",
            "   │ ",
            " ╭─╯ ",
            " │   ",
            " ╰── ",
        ],
        '3' => [
            " ╭─╮ ",
            "   │ ",
            "  ─┤ ",
            "   │ ",
            " ╰─╯ ",
        ],
        '4' => [
            " ╷ ╷ ",
            " │ │ ",
            " ╰─┤ ",
            "   │ ",
            "   ╵ ",
        ],
        '5' => [
            " ╭── ",
            " │   ",
            " ╰─╮ ",
            "   │ ",
            " ──╯ ",
        ],
        '6' => [
            " ╭─╮ ",
            " │   ",
            " ├─╮ ",
            " │ │ ",
            " ╰─╯ ",
        ],
        '7' => [
            " ──╮ ",
            "   │ ",
            "  ╱  ",
            " ╱   ",
            " ╵   ",
        ],
        '8' => [
            " ╭─╮ ",
            " │ │ ",
            " ├─┤ ",
            " │ │ ",
            " ╰─╯ ",
        ],
        '9' => [
            " ╭─╮ ",
            " │ │ ",
            " ╰─┤ ",
            "   │ ",
            " ╰─╯ ",
        ],
        ':' => [
            "     ",
            "  ◦  ",
            "     ",
            "  ◦  ",
            "     ",
        ],
        _ => [
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ],
    }
}

/// Small digit font - 3 lines tall, 3 chars wide (for narrow terminals)
pub fn small_digit(ch: char) -> [&'static str; 3] {
    match ch {
        '0' => ["╭─╮", "│ │", "╰─╯"],
        '1' => [" ╷ ", " │ ", " ╵ "],
        '2' => ["╭─╮", "╭─╯", "╰──"],
        '3' => ["──╮", " ─┤", "──╯"],
        '4' => ["╷ ╷", "╰─┤", "  ╵"],
        '5' => ["╭──", "╰─╮", "──╯"],
        '6' => ["╭─╮", "├─╮", "╰─╯"],
        '7' => ["──╮", "  │", "  ╵"],
        '8' => ["╭─╮", "├─┤", "╰─╯"],
        '9' => ["╭─╮", "╰─┤", "╰─╯"],
        ':' => ["   ", " ◦ ", " ◦ "],
        _ => ["   ", "   ", "   "],
    }
}

/// Render time string into big text lines (adaptive to width)
pub fn render_big_time(time: &str, max_width: u16) -> Vec<String> {
    let use_big = max_width >= 34;
    let height = if use_big { 5 } else { 3 };
    let mut lines = vec![String::new(); height];

    let chars: Vec<char> = time.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if use_big {
            let digit = big_digit(*ch);
            for (idx, line) in digit.iter().enumerate() {
                lines[idx].push_str(line);
            }
        } else {
            let digit = small_digit(*ch);
            for (idx, line) in digit.iter().enumerate() {
                lines[idx].push_str(line);
                if i < chars.len() - 1 {
                    lines[idx].push(' ');
                }
            }
        }
    }
    lines
}

/// ASCII art for each state - 5 lines tall
pub fn state_art(state: &str) -> Vec<&'static str> {
    match state {
        "studying" => vec![
            r"    ___    ",
            r"   /   \   ",
            r"  | ~~~ |  ",
            r"  | ~~~ |  ",
            r"   \___/   ",
        ],
        "break" => vec![
            r"   ) )  )  ",
            r"  ( ( (    ",
            r"  ┌─────┐  ",
            r"  │     │) ",
            r"  └─────┘  ",
        ],
        "paused" => vec![
            r"           ",
            r"  ██  ██   ",
            r"  ██  ██   ",
            r"  ██  ██   ",
            r"           ",
        ],
        "idle" => vec![
            r"    *      ",
            r"   /|\     ",
            r"  * | *    ",
            r"   \|/     ",
            r"    *      ",
        ],
        "done" => vec![
            r"           ",
            r"       ╱   ",
            r"     ╱     ",
            r"  ╲╱       ",
            r"           ",
        ],
        _ => vec![
            "           ",
            "           ",
            "           ",
            "           ",
            "           ",
        ],
    }
}
