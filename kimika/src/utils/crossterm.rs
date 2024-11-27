use crossterm::tty::IsTty;
use crossterm::{cursor, execute, terminal};
use std::io::Read;

/// use `std::io::stdout`
pub fn clear_up_lines(lines: u16) -> Result<(), std::io::Error> {
    execute!(
        std::io::stdout(),
        cursor::MoveToPreviousLine(lines),
        terminal::Clear(terminal::ClearType::FromCursorDown),
    )
}

pub fn try_read_from_pipeline() -> Option<Vec<String>> {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    if stdin.is_tty() {
        None
    } else {
        stdin.read_to_string(&mut input).unwrap();
        Some(vec![input.trim_end().to_string()])
    }
}
